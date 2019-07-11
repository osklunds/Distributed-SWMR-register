
use std::str;
use std::sync::{Arc, Mutex, Condvar, Weak};
use std::collections::HashSet;
use std::borrow::Cow;
use std::fmt::Debug;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::register::*;
use crate::entry::{Entry, Timestamp};
use crate::messages::*;
use crate::terminal_output::printlnu;
use crate::mediator::Mediator;


pub struct AbdNode<V> {
    mediator: Weak<Mediator>,

    ts: Arc<Mutex<Timestamp>>,
    reg: Arc<Mutex<Register<V>>>,

    id: Arc<NodeId>,
    node_ids: Arc<HashSet<NodeId>>,

    acking_processors_for_write: Arc<Mutex<HashSet<NodeId>>>,
    register_being_written: Arc<Mutex<Option<Register<V>>>>,
    write_ack_majority_reached: Arc<Condvar>,

    acking_processors_for_read: Arc<Mutex<HashSet<NodeId>>>,
    register_being_read: Arc<Mutex<Option<Register<V>>>>  
}

impl<V: Default + Serialize + DeserializeOwned + Debug + Clone> AbdNode<V> {
    pub fn new(node_id: NodeId, node_ids: HashSet<NodeId>, mediator: Weak<Mediator>) -> AbdNode<V> {
        AbdNode {
            mediator: mediator,
            ts: Arc::new(Mutex::new(-1)),
            reg: Arc::new(Mutex::new(Register::new(&node_ids))),
            id: Arc::new(node_id),
            node_ids: Arc::new(node_ids),
            acking_processors_for_write: Arc::new(Mutex::new(HashSet::new())),
            register_being_written: Arc::new(Mutex::new(None)),
            write_ack_majority_reached: Arc::new(Condvar::new()),
            acking_processors_for_read: Arc::new(Mutex::new(HashSet::new())),
            register_being_read: Arc::new(Mutex::new(None)),
        }
    }

    pub fn json_received(&self, json: &str) {
        if self.json_string_is_write_message(json) {
            if let Ok(write_message) = serde_json::from_str(&json) {
                return self.receive_write_message(write_message);
            }
        }

        if self.json_string_is_write_ack_message(json) {
            if let Ok(write_ack_message) = serde_json::from_str(&json) {
                return self.receive_write_ack_message(write_ack_message);
            }
        }

        if self.json_string_is_read_message(json) {
            if let Ok(read_message) = serde_json::from_str(&json) {
                return self.receive_read_message(read_message);
            }
        }

        if self.json_string_is_read_ack_message(json) {
            if let Ok(read_ack_message) = serde_json::from_str(&json) {
                return self.receive_read_ack_message(read_ack_message);
            }
        }

        printlnu(format!("Could not parse the message {}", json));
    }

    fn json_string_is_write_message(&self, json: &str) -> bool {
        json.starts_with("{\"WriteMessage\":")
    }

    fn json_string_is_write_ack_message(&self, json: &str) -> bool {
        json.starts_with("{\"WriteAckMessage\":")
    }

    fn json_string_is_read_message(&self, json: &str) -> bool {
        json.starts_with("{\"ReadMessage\":")
    }

    fn json_string_is_read_ack_message(&self, json: &str) -> bool {
        json.starts_with("{\"ReadAckMessage\":")
    }

    fn receive_write_message(&self, write_message: WriteMessage<V>) {
        let write_ack_message;

        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(&write_message.register);

            write_ack_message = WriteAckMessage {
                sender: *self.id,
                register: Cow::Borrowed(&reg)
            };

            self.send_message_to(&write_ack_message, write_message.sender);
        }
    }

    fn receive_write_ack_message(&self, write_ack_message: WriteAckMessage<V>) {
        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(&write_ack_message.register);
        }

        let mut register_being_written = self.register_being_written.lock().unwrap();
        let mut new_reg = false;
        let mut majority_reached = false;

        if let Some(register_being_written) = &*register_being_written {
            if *write_ack_message.register >= *register_being_written {
                let mut acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
                acking_processors_for_write.insert(write_ack_message.sender);

                new_reg = true;
            }
        }

        if new_reg && self.write_ack_from_majority() {
            majority_reached = true;
            let mut acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
            acking_processors_for_write.clear();
        }

        if majority_reached {
            *register_being_written = None;
            self.write_ack_majority_reached.notify_one();
        }
    }

    fn receive_read_message(&self, read_message: ReadMessage<V>) {
        let read_ack_message;

        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(&read_message.register);

            read_ack_message = ReadAckMessage {
                sender: *self.id,
                register: Cow::Borrowed(&reg)
            };

            self.send_message_to(&read_ack_message, read_message.sender);
        } 
    }

    fn receive_read_ack_message(&self, read_ack_message: ReadAckMessage<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register(&read_ack_message.register);

        let mut register_being_read = self.register_being_read.lock().unwrap();
        if let Some(register_being_read) = &*register_being_read {
            if *read_ack_message.register >= *register_being_read {
                let mut acking_processors_for_read = self.acking_processors_for_read.lock().unwrap();
                acking_processors_for_read.insert(read_ack_message.sender);
            }

            // TODO: Send () on a channel here
        }
        else {
            println!("Tomt");
        }
    }


    fn send_message_to(&self, message: &impl Message, receiver_id: NodeId) {
        let json = serde_json::to_string(message).unwrap();
        self.mediator().send_json_to(&json, receiver_id);
    }

    fn broadcast_message(&self, message: &impl Message) {
        let json = serde_json::to_string(message).unwrap();
        self.mediator().broadcast_json(&json);
    }
    
    fn mediator(&self) -> Arc<Mediator> {
        self.mediator.upgrade().unwrap()
    }

    pub fn write(&self, value: V) {
        //printlnu(format!("Start write {:?}", &value));
        let value2 = value.clone();

        let write_message;
        let reg_to_write;

        {
            let mut ts = self.ts.lock().unwrap();
            let mut reg = self.reg.lock().unwrap();

            *ts += 1;
            reg.set(*self.id, Entry::new(*ts, value));

            reg_to_write = reg.clone();

            write_message = WriteMessage {
                sender: *self.id,
                register: Cow::Borrowed(&reg_to_write)
            };

            let mut register_being_written = self.register_being_written.lock().unwrap();
            *register_being_written = Some(reg.clone());

            self.broadcast_message(&write_message);
        }

        {
            let mut register_being_written = self.register_being_written.lock().unwrap();

            while register_being_written.is_some() {
                register_being_written = self.write_ack_majority_reached.wait(register_being_written).unwrap();
            }
        }

        
        let mut register_being_written = self.register_being_written.lock().unwrap();
        *register_being_written = None;
        let mut acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
        acking_processors_for_write.clear();

        //printlnu(format!("End write {:?}", &value2));
        
    }
    
    fn write_ack_from_majority(&self) -> bool {
        let acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();

        acking_processors_for_write.len() >= self.number_of_nodes_in_a_majority()
    }

    fn number_of_nodes_in_a_majority(&self) -> usize {
        self.number_of_nodes() / 2 + 1
    }

    fn number_of_nodes(&self) -> usize {
        self.node_ids.len()
    }
}