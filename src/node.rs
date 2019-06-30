
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use std::collections::{HashMap, HashSet};

use serde_json;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use std::fmt::Debug;

use crate::register::Register;
use crate::register::Entry;
use crate::message::*;


pub struct Node<V> {
    id: Arc<i32>,
    ts: Arc<Mutex<i32>>,
    reg: Arc<Mutex<Register<V>>>,
    socket: Arc<UdpSocket>,
    socket_addrs: Arc<HashMap<i32, SocketAddr>>
}

impl<V: Default + Serialize + DeserializeOwned + Debug + Clone + Ord> Node<V> {
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
            socket_addrs: Arc::new(socket_addrs)
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
        match &message.message_type {
            MessageType::WriteMessage => self.handle_write_message(&message),
            MessageType::WriteAckMessage => self.handle_write_ack_message(&message),
            MessageType::ReadMessage => self.handle_read_message(&message),
            MessageType::ReadAckMessage => self.handle_read_ack_message(&message)
        };
    }

    fn handle_write_message(&self, message: &Message<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register(&message.register);


        println!("{:?}", *reg);
    }

    fn handle_write_ack_message(&self, message: &Message<V>) {

    }


    fn handle_read_message(&self, message: &Message<V>) {

    }


    fn handle_read_ack_message(&self, message: &Message<V>) {

    }


    fn send_message_to(&self, message: &Message<V>, receiver_id: i32) {
        let json_string = serde_json::to_string(message).unwrap();
        let bytes = json_string.as_bytes();
        let dst_socket_addr = self.socket_addrs.get(&receiver_id).unwrap();
        println!("Skickar till {:?}", dst_socket_addr);
        self.socket.send_to(bytes, dst_socket_addr).unwrap();
    }

    pub fn client_op_loop(&self) {
        /*
        loop {
            let next_id = match *self.id {
                1 => 2,
                2 => 1,
                _ => panic!("Bad id")
            };

            let mess: Message<V> = Message {
                sender: *self.id,
                message_type: MessageType::WriteMessage,
                register: Register::new(HashSet::new())
            };

            self.send_message_to(&mess, next_id);

            thread::sleep(time::Duration::from_millis(50000));
        }
        */

        if *self.id == 1 {
            let mut reg = self.reg.lock().unwrap();
            reg.set(1, Entry::new(7, V::default()));

            thread::sleep(time::Duration::from_millis(500));

            let mess: Message<V> = Message {
                sender: *self.id,
                message_type: MessageType::WriteMessage,
                register: reg.clone()
            };

            self.send_message_to(&mess, 2);


        }



    }


}