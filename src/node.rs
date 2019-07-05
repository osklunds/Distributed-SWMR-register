
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;

use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::channel;
use std::{thread, time};

use std::collections::{HashMap, HashSet};

use serde_json;
use serde_json::Error;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use std::fmt::Debug;
use std::hash::Hash;

use std::borrow::Cow;

use crate::register::Register;
use crate::register::Entry;
use crate::messages::*;


pub struct Node<'a, V> {
    ts: Arc<Mutex<i32>>,
    reg: Arc<Mutex<Register<V>>>,

    id: Arc<i32>,
    socket: Arc<UdpSocket>,
    socket_addrs: Arc<HashMap<i32, SocketAddr>>,

    acking_processors_for_write: Arc<Mutex<HashSet<i32>>>,
    register_being_written: Arc<Mutex<Option<Register<V>>>>,

    acking_processors_for_read: Arc<Mutex<HashSet<i32>>>,
    register_being_read: Arc<Mutex<Option<&'a Register<V>>>>,
}

impl<'a, V: Default + Serialize + DeserializeOwned + Debug + Clone + Ord + Eq + Hash> Node<'a, V> {
    pub fn new(node_id: i32, socket_addrs: HashMap<i32, SocketAddr>) -> Node<'a, V> {
        let my_socket_addr = socket_addrs.get(&node_id).unwrap();
        let socket = UdpSocket::bind(my_socket_addr).unwrap();

        let mut node_ids = HashSet::new();
        for key in socket_addrs.keys() {
            node_ids.insert(*key);
        }

        Node {
            id: Arc::new(node_id),
            ts: Arc::new(Mutex::new(-1)),
            reg: Arc::new(Mutex::new(Register::new(node_ids))),
            socket: Arc::new(socket),
            socket_addrs: Arc::new(socket_addrs),
            acking_processors_for_write: Arc::new(Mutex::new(HashSet::new())),
            register_being_written: Arc::new(Mutex::new(None)),
            acking_processors_for_read: Arc::new(Mutex::new(HashSet::new())),
            register_being_read: Arc::new(Mutex::new(None)),
        }
    }

    pub fn recv_loop(&self) {
        loop {
            let mut buf = [0; 4096];

            let (amt, socket_addr) = self.socket.recv_from(&mut buf).unwrap();
            let json_string = str::from_utf8(&buf[0..amt]).unwrap();

            self.handle_message_json_string(json_string);

            //self.handle_message(message);
        }
    }

    fn handle_message_json_string(&self, json: &str) {
        let write_message: Result<WriteMessage<V>, Error> = serde_json::from_str(&json);
        let write_ack_message: Result<WriteAckMessage<V>, Error> = serde_json::from_str(&json);
        let read_message: Result<ReadMessage<V>, Error> = serde_json::from_str(&json);
        let read_ack_message: Result<ReadAckMessage<V>, Error> = serde_json::from_str(&json);

        if let Ok(write_message) = write_message {
            self.handle_write_message(write_message);
        } else if let Ok(write_ack_message) = write_ack_message {
            self.handle_write_ack_message(write_ack_message);
        } else if let Ok(read_message) = read_message {
            self.handle_read_message(read_message);
        } else if let Ok(read_ack_message) = read_ack_message {
            self.handle_read_ack_message(read_ack_message);
        } else {
            println!("Could not deserialize: {}",json);
        }
    }

    fn handle_write_message(&self, write_message: WriteMessage<V>) {
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

    fn handle_write_ack_message(&self, write_ack_message: WriteAckMessage<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register(&write_ack_message.register);

        let mut register_being_written = self.register_being_written.lock().unwrap();
        if let Some(register_being_written) = &*register_being_written {
            if *write_ack_message.register >= *register_being_written {
                let mut acking_processors_for_write = self.acking_processors_for_write.lock().unwrap();
                acking_processors_for_write.insert(write_ack_message.sender);
            }

            // TODO: Send () on a channel here
        }
    }

    fn handle_read_message(&self, read_message: ReadMessage<V>) {
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

    fn handle_read_ack_message(&self, read_ack_message: ReadAckMessage<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register(&read_ack_message.register);

        let mut register_being_read = self.register_being_read.lock().unwrap();
        if let Some(register_being_read) = *register_being_read {
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

    fn send_message_to(&self, message: &impl Message, receiver_id: i32) {
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
                register: Cow::Borrowed(&reg)
            };

            let mut register_being_written = self.register_being_written.lock().unwrap();
            *register_being_written = Some(reg_to_write);

            self.broadcast_message(&write_message);
        }

        while !self.write_ack_from_majority() {
            thread::sleep(time::Duration::from_millis(5));
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