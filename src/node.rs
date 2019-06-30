
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use std::collections::{HashMap, HashSet};

use crate::register;

pub struct Node<V> {
    id: Arc<i32>,
    ts: Arc<Mutex<i32>>,
    reg: Arc<Mutex<register::Register<V>>>,
    socket: Arc<UdpSocket>
}

impl<V: Default> Node<V> {
    pub fn new(node_id: i32, node_ids: HashSet<i32>) -> Node<V> {
        let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

        Node {
            id: Arc::new(node_id),
            ts: Arc::new(Mutex::new(-1)),
            reg: Arc::new(Mutex::new(register::Register::new(node_ids))),
            socket: Arc::new(socket)
        }
    }


}