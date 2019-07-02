
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;

use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::channel;
use std::{thread, time};

use std::collections::{HashMap, HashSet};

use serde_json;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use std::fmt::Debug;
use std::hash::Hash;

use crate::register::Register;
use crate::register::Entry;
use crate::message::*;


pub struct Node<V> {
    id: Arc<i32>,
    ts: Arc<Mutex<i32>>,
    reg: Arc<Mutex<Register<V>>>,
    socket: Arc<UdpSocket>,
    socket_addrs: Arc<HashMap<i32, SocketAddr>>,
    write_ack_message_bag: Arc<Mutex<HashSet<Message<V>>>>,
    read_ack_message_bag: Arc<Mutex<HashSet<Message<V>>>>,
    write_ack_from_majority_cond: Arc<Condvar>
}

impl<V: Default + Serialize + DeserializeOwned + Debug + Clone + Ord + Eq + Hash> Node<V> {
    pub fn new(node_id: i32, socket_addrs: HashMap<i32, SocketAddr>) -> Node<V> {
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
            write_ack_message_bag: Arc::new(Mutex::new(HashSet::new())),
            read_ack_message_bag: Arc::new(Mutex::new(HashSet::new())),
            write_ack_from_majority_cond: Arc::new(Condvar::new())
        }
    }

    pub fn recv_loop(&self) {
        loop {
            let mut buf = [0; 4096];

            let (amt, socket_addr) = self.socket.recv_from(&mut buf).unwrap();
            let json_string = str::from_utf8(&buf[0..amt]).unwrap();
            let message: Message<V> = serde_json::from_str(&json_string).unwrap();

            self.handle_message(message);
        }
    }

    fn handle_message(&self, message: Message<V>) {
        //println!("Ska hantera {:?}", message);

        match &message.message_type {
            MessageType::WriteMessage => self.handle_write_message(&message),
            MessageType::WriteAckMessage => self.handle_write_ack_message(&message),
            MessageType::ReadMessage => self.handle_read_message(&message),
            MessageType::ReadAckMessage => self.handle_read_ack_message(&message)
        };
    }

    fn handle_write_message(&self, message: &Message<V>) {
        let write_ack_message: Message<V>;

        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(&message.register);

            //println!("{:?}", *reg);

            write_ack_message = Message {
                    sender: *self.id,
                    message_type: MessageType::WriteAckMessage,
                    register: reg.clone()
            };
        }

        self.send_message_to(&write_ack_message, message.sender);
    }

    fn handle_write_ack_message(&self, message: &Message<V>) {
        let mut write_ack_message_bag = self.write_ack_message_bag.lock().unwrap();

        write_ack_message_bag.insert(message.clone());
    }

    fn handle_read_message(&self, message: &Message<V>) {
        let read_ack_message: Message<V>;

        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register(&message.register);

            read_ack_message = Message {
                    sender: *self.id,
                    message_type: MessageType::ReadAckMessage,
                    register: reg.clone()
            };
        }

        self.send_message_to(&read_ack_message, message.sender);
    }

    fn handle_read_ack_message(&self, message: &Message<V>) {
        let mut read_ack_message_bag = self.read_ack_message_bag.lock().unwrap();

        read_ack_message_bag.insert(message.clone());
    }


    fn send_message_to(&self, message: &Message<V>, receiver_id: i32) {
        let json_string = serde_json::to_string(message).unwrap();
        let bytes = json_string.as_bytes();
        let dst_socket_addr = self.socket_addrs.get(&receiver_id).unwrap();
        //println!("Skickar {:?} till {:?}", message, dst_socket_addr);
        self.socket.send_to(bytes, dst_socket_addr).unwrap();
    }

    fn broadcast_message(&self, message: &Message<V>) {
        for node_id in self.socket_addrs.keys() {
            self.send_message_to(message, *node_id);
        }
    }

    pub fn write(&self, value: V) {
        //println!("Start write {:?}", &value);
        let value2 = value.clone();

        let write_message = self.update_reg(value);

        self.broadcast_message(&write_message);

        while !self.write_ack_from_majority() {
            //thread::sleep(time::Duration::from_millis(50));
        }

        let mut write_ack_message_bag = self.write_ack_message_bag.lock().unwrap();
        write_ack_message_bag.clear();

        //println!("End write {:?}", &value2);
    }

    pub fn update_reg(&self, value: V) -> Message<V> {
        let mut ts = self.ts.lock().unwrap();
        let mut reg = self.reg.lock().unwrap();

        *ts += 1;
        reg.set(*self.id, Entry::new(*ts, value));

        let write_message: Message<V> = Message {
            sender: *self.id,
            message_type: MessageType::WriteMessage,
            register: reg.clone()
        };

        return write_message;
    }

    fn write_ack_from_majority(&self) -> bool {
        let maj = 3; // Just temp

        let mut acking_processors = 0;

        let mut reg = self.reg.lock().unwrap();
        let mut write_ack_message_bag = self.write_ack_message_bag.lock().unwrap();

        for message in write_ack_message_bag.iter() {
            if message.register >= *reg {
                acking_processors += 1;
            }
        }

        acking_processors >= maj
    }

    pub fn client_op_loop(&self) {

    }


}