
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::io;
use std::str;

use std::sync::{Arc, Mutex, MutexGuard, Condvar};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, time};

use std::collections::{HashMap, HashSet};

use serde_json;
use serde_json::Error;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use std::fmt::Debug;
use std::hash::Hash;

use std::borrow::Cow;

use crate::register::*;
use crate::entry::{Entry, Timestamp};
use crate::messages::*;
use crate::terminal_output::printlnu;


pub struct AbdNode<V> {
    ts: Arc<Mutex<Timestamp>>,
    reg: Arc<Mutex<Register<V>>>,

    id: Arc<NodeId>,
    socket: Arc<UdpSocket>,
    socket_addrs: Arc<HashMap<NodeId, SocketAddr>>,

    acking_processors_for_write: Arc<Mutex<HashSet<NodeId>>>,
    register_being_written: Arc<Mutex<Option<Register<V>>>>,
    write_ack_majority_reached: Arc<Condvar>,

    acking_processors_for_read: Arc<Mutex<HashSet<NodeId>>>,
    register_being_read: Arc<Mutex<Option<Register<V>>>>  
}

impl<V: Default + Serialize + DeserializeOwned + Debug + Clone> AbdNode<V> {
    pub fn new(node_id: NodeId, socket_addrs: HashMap<NodeId, SocketAddr>) -> io::Result<AbdNode<V>> {
        let my_socket_addr = socket_addrs.get(&node_id).expect("My node id was not included among the socket addresses.");
        let socket = UdpSocket::bind(my_socket_addr)?;

        let mut node_ids = HashSet::new();
        for key in socket_addrs.keys() {
            node_ids.insert(*key);
        }

        Ok(AbdNode {
            id: Arc::new(node_id),
            ts: Arc::new(Mutex::new(-1)),
            reg: Arc::new(Mutex::new(Register::new(node_ids))),
            socket: Arc::new(socket),
            socket_addrs: Arc::new(socket_addrs),
            acking_processors_for_write: Arc::new(Mutex::new(HashSet::new())),
            register_being_written: Arc::new(Mutex::new(None)),
            write_ack_majority_reached: Arc::new(Condvar::new()),
            acking_processors_for_read: Arc::new(Mutex::new(HashSet::new())),
            register_being_read: Arc::new(Mutex::new(None)),
        })
    }

    pub fn recv_loop(&self) {
        loop {
            let mut buf = [0; 4096];

            let (amt, socket_addr) = self.socket.recv_from(&mut buf).unwrap();
            let json_string = str::from_utf8(&buf[0..amt]).unwrap();

            self.receive_json_string(json_string);
        }
    }

    fn receive_json_string(&self, json: &str) {
        if let Ok(w) = serde_json::from_str(&json) {
            let x: WriteMessage<V> = w;
        }

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
        let json_string = serde_json::to_string(message).unwrap();
        let bytes = json_string.as_bytes();
        let dst_socket_addr = self.socket_addrs.get(&receiver_id).unwrap();
        self.socket.send_to(bytes, dst_socket_addr).unwrap();
    }

    fn broadcast_message(&self, message: &impl Message) {
        for node_id in self.socket_addrs.keys() {
            self.send_message_to(message, *node_id);
        }
    }
    
    pub fn write(&self, value: V) {
        //println!("Start write {:?}", &value);
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

        //println!("End write {:?}", &value2);
    }
    
    fn write_ack_from_majority(&self) -> bool {
        let acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();

        acking_processors_for_write.len() >= self.number_of_nodes_in_a_majority()
    }

    fn number_of_nodes_in_a_majority(&self) -> usize {
        self.number_of_nodes() / 2 + 1
    }

    fn number_of_nodes(&self) -> usize {
        self.socket_addrs.len()
    }
}