
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::io;
use std::str;
use std::sync::{Arc, Weak};
use std::collections::HashMap;
use std::fmt::Debug;


use serde_json;


use crate::register::NodeId;
use crate::terminal_output::printlnu;
use crate::mediator::Mediator;


pub struct Communicator {
    pub id: NodeId,
    socket: UdpSocket,
    socket_addrs: HashMap<NodeId, SocketAddr>,
    mediator: Weak<Mediator>
}

impl Communicator {
    pub fn new(node_id: NodeId, socket_addrs: HashMap<NodeId, SocketAddr>, mediator: Weak<Mediator>) -> io::Result<Communicator> {
        let my_socket_addr = socket_addrs.get(&node_id).expect("My node id was not included among the socket addresses.");
        let socket = UdpSocket::bind(my_socket_addr)?;

        Ok(Communicator {
            id: node_id,
            socket: socket,
            socket_addrs: socket_addrs,
            mediator: mediator
        })
    }

    pub fn recv_loop(&self) {
        loop {
            let mut buf = [0; 4096];

            let (amt, socket_addr) = self.socket.recv_from(&mut buf).unwrap();
            let json_string = str::from_utf8(&buf[0..amt]).unwrap();

            self.mediator().json_received(json_string);
        }
    }

    fn mediator(&self) -> Arc<Mediator> {
        self.mediator.upgrade().unwrap()
    }
    

    pub fn send_json_to(&self, json: &str, receiver_id: NodeId) {
        let bytes = json.as_bytes();
        let dst_socket_addr = self.socket_addrs.get(&receiver_id).unwrap();
        self.socket.send_to(bytes, dst_socket_addr).unwrap();
    }

    pub fn broadcast_json(&self, json: &str) {
        let bytes = json.as_bytes();
        for (_, socket_addr) in self.socket_addrs.iter() {
            self.socket.send_to(bytes, socket_addr).unwrap();
        }
    }
}