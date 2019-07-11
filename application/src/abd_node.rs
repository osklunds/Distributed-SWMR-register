
use std::str;
use std::sync::{Arc, Mutex, Condvar, Weak};
use std::collections::HashSet;
use std::borrow::Cow;
use std::fmt::Debug;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::settings::{SETTINGS, NodeId};
use crate::register::*;
use crate::entry::{self, Entry, Timestamp};
use crate::messages::*;
use crate::terminal_output::printlnu;
use crate::mediator::Mediator;


pub struct AbdNode<V> {
    mediator: Weak<Mediator>,

    ts: Mutex<Timestamp>,
    reg: Mutex<Register<V>>,

    register_being_written: Mutex<Option<Register<V>>>,
    acking_processors_for_write: Mutex<HashSet<NodeId>>,
    write_ack_majority_reached: Condvar,

    register_being_read: Mutex<Option<Register<V>>>,
    acking_processors_for_read: Mutex<HashSet<NodeId>>,
    read_ack_majority_reached: Condvar
}

impl<V: Default + Serialize + DeserializeOwned + Debug + Clone> AbdNode<V> {
    pub fn new(mediator: Weak<Mediator>) -> AbdNode<V> {
        AbdNode {
            mediator: mediator,
            ts: Mutex::new(entry::default_timestamp()),
            reg: Mutex::new(Register::new(&SETTINGS.node_ids())),
            
            register_being_written: Mutex::new(None),
            acking_processors_for_write: Mutex::new(HashSet::new()),
            write_ack_majority_reached: Condvar::new(),
            
            register_being_read: Mutex::new(None),
            acking_processors_for_read: Mutex::new(HashSet::new()),
            read_ack_majority_reached: Condvar::new()
        }
    }

    pub fn write(&self, value: V) {
        let mut value2 = None;
        if cfg!(debug_assertions) {
            if SETTINGS.print_start_end_of_client_operations() {
                printlnu(format!("Start write {:?}", &value));
            }
            value2 = Some(value.clone());
        }

        let json_write_message;

        {
            let mut ts = self.ts.lock().unwrap();
            let mut reg = self.reg.lock().unwrap();

            *ts += 1;
            reg.set(SETTINGS.node_id(), Entry::new(*ts, value));

            let mut register_being_written = self.register_being_written.lock().unwrap();

            if cfg!(debug_assertions) {
                assert_eq!(*register_being_written, None);
            }

            *register_being_written = Some(reg.clone());

            let write_message = WriteMessage {
                sender: SETTINGS.node_id(),
                register: Cow::Borrowed(&reg)
            };

            json_write_message = self.jsonify_message(&write_message);
        }

        self.broadcast_json_message(&json_write_message);
            
        {
            let mut register_being_written = self.register_being_written.lock().unwrap();

            while register_being_written.is_some() {
                register_being_written = self.write_ack_majority_reached.wait(register_being_written).unwrap();
            }
        }

        if cfg!(debug_assertions) {
            if SETTINGS.print_start_end_of_client_operations() {
                printlnu(format!("End write {:?}", &value2.unwrap()));
            }

            let acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
            let register_being_written = self.register_being_written.lock().unwrap();
            assert!(acking_processors_for_write.is_empty());
            assert!(register_being_written.is_none());
        }
    }

    fn jsonify_message(&self, message: &impl Message) -> String {
        serde_json::to_string(message).unwrap()
    }

    fn broadcast_json_message(&self, json: &str) {
        self.mediator().broadcast_json(json);
    }

    fn mediator(&self) -> Arc<Mediator> {
        self.mediator.upgrade().unwrap()
    }

    pub fn json_received(&self, json: &str) {
        if self.json_is_write_message(json) {
            if let Ok(write_message) = serde_json::from_str(&json) {
                return self.receive_write_message(write_message);
            }
        }

        if self.json_is_write_ack_message(json) {
            if let Ok(write_ack_message) = serde_json::from_str(&json) {
                return self.receive_write_ack_message(write_ack_message);
            }
        }

        if self.json_is_read_message(json) {
            if let Ok(read_message) = serde_json::from_str(&json) {
                return self.receive_read_message(read_message);
            }
        }

        if self.json_is_read_ack_message(json) {
            if let Ok(read_ack_message) = serde_json::from_str(&json) {
                return self.receive_read_ack_message(read_ack_message);
            }
        }

        printlnu(format!("Could not parse the json: {}", json));
    }

    fn json_is_write_message(&self, json: &str) -> bool {
        json.starts_with("{\"WriteMessage\":")
    }

    fn json_is_write_ack_message(&self, json: &str) -> bool {
        json.starts_with("{\"WriteAckMessage\":")
    }

    fn json_is_read_message(&self, json: &str) -> bool {
        json.starts_with("{\"ReadMessage\":")
    }

    fn json_is_read_ack_message(&self, json: &str) -> bool {
        json.starts_with("{\"ReadAckMessage\":")
    }

    fn receive_write_message(&self, write_message: WriteMessage<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register(&write_message.register);

        let write_ack_message = WriteAckMessage {
            sender: SETTINGS.node_id(),
            register: Cow::Borrowed(&reg)
        };

        self.send_message_to(&write_ack_message, write_message.sender);

        // Here we have a compromise. Either we lock reg for
        // a long time, or we clone reg so we can have more
        // concurrency. For small entries, cloning might be
        // better. For large entries, longer locking
        // might be better.
    }

    fn receive_write_ack_message(&self, write_ack_message: WriteAckMessage<V>) {
        let received_register: &Register<V> = &write_ack_message.register;        
        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(received_register);
        }

        let mut register_being_written = self.register_being_written.lock().unwrap();
        let mut received_register_was_at_least_as_large = false;
        
        if let Some(register_being_written) = &*register_being_written {
            if received_register >= register_being_written {
                let mut acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
                acking_processors_for_write.insert(write_ack_message.sender);

                received_register_was_at_least_as_large = true;
            }
        }

        if received_register_was_at_least_as_large && self.write_ack_from_majority() {
            let mut acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
            acking_processors_for_write.clear();
            *register_being_written = None;

            self.write_ack_majority_reached.notify_one();
        }
    }

    fn receive_read_message(&self, read_message: ReadMessage<V>) {    
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register(&read_message.register);

        let read_ack_message = ReadAckMessage {
            sender: SETTINGS.node_id(),
            register: Cow::Borrowed(&reg)
        };

        self.send_message_to(&read_ack_message, read_message.sender);
    }

    fn receive_read_ack_message(&self, read_ack_message: ReadAckMessage<V>) {
        let received_register: &Register<V> = &read_ack_message.register;
        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(&received_register);
        }

        let mut register_being_read = self.register_being_read.lock().unwrap();
        let mut received_register_was_at_least_as_large = false;
        
        if let Some(register_being_read) = &*register_being_read {
            if received_register >= register_being_read {
                let mut acking_processors_for_read = self.acking_processors_for_read.lock().unwrap();
                acking_processors_for_read.insert(read_ack_message.sender);

                received_register_was_at_least_as_large = true;
            }
        }

        if received_register_was_at_least_as_large && self.read_ack_from_majority() {
            let mut acking_processors_for_read = self.acking_processors_for_read.lock().unwrap();
            acking_processors_for_read.clear();

            *register_being_read = None;
            self.read_ack_majority_reached.notify_one();
        }
    }

    fn write_ack_from_majority(&self) -> bool {
        let acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();

        acking_processors_for_write.len() >= self.number_of_nodes_in_a_majority()
    }

    fn read_ack_from_majority(&self) -> bool {
        let acking_processors_for_read = self.acking_processors_for_read.lock().unwrap();

        acking_processors_for_read.len() >= self.number_of_nodes_in_a_majority()
    }

    fn number_of_nodes_in_a_majority(&self) -> usize {
        SETTINGS.number_of_nodes() / 2 + 1
    }

    fn send_message_to(&self, message: &impl Message, receiver_id: NodeId) {
        let json = serde_json::to_string(message).unwrap();
        self.mediator().send_json_to(&json, receiver_id);
    }    
}