
use std::net::UdpSocket;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::str;
use std::sync::{Arc, Weak}; 
use std::thread;
use std::collections::HashMap;

use crate::settings::NodeId;
use crate::mediator::Mediator;


pub struct Communicator {
    socket: UdpSocket,
    socket_addrs: HashMap<NodeId, SocketAddr>,
    mediator: Weak<Mediator>
}

impl Communicator {
    pub fn new(own_socket_addr: SocketAddr, socket_addrs: HashMap<NodeId, SocketAddr>, mediator: Weak<Mediator>) -> Arc<Communicator> {
        let own_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), own_socket_addr.port());
        let socket = UdpSocket::bind(own_socket_addr).expect("Could not create socket.");

        let communicator = Communicator {
            socket: socket,
            socket_addrs: socket_addrs,
            mediator: mediator
        };
        let communicator = Arc::new(communicator);
        let recv_thread_communicator = Arc::clone(&communicator);
        thread::spawn(move || {
            recv_thread_communicator.recv_loop();
        });
        communicator
    }

    pub fn recv_loop(&self) {
        loop {
            let mut buf = [0; 4096];

            let amt = self.socket.recv(&mut buf).unwrap();
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
        for socket_addr in self.socket_addrs.values() {
            self.socket.send_to(bytes, socket_addr).unwrap();
        }
    }
}