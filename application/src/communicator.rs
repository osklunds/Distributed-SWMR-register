
use std::net::UdpSocket;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::str;
use std::sync::{Arc, Weak}; 
use std::thread;
use std::collections::HashMap;

use commons::types::NodeId;

use crate::mediator::Med;


pub struct Communicator<M> {
    socket: UdpSocket,
    socket_addrs: HashMap<NodeId, SocketAddr>,
    mediator: Weak<M>
}

impl<M: Med> Communicator<M> {
    pub fn new(own_socket_addr: SocketAddr, socket_addrs: HashMap<NodeId, SocketAddr>, mediator: Weak<M>) -> Arc<Communicator<M>> {
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

            let amt = self.socket.recv(&mut buf).expect("Error receiving from socket");
            let json_string = str::from_utf8(&buf[0..amt]).expect("Error converting bytes to utf8");

            self.mediator().json_received(json_string);
        }
    }

    fn mediator(&self) -> Arc<M> {
        self.mediator.upgrade().expect("Error upgrading mediator in Communicator")
    }

    pub fn send_json_to(&self, json: &str, receiver_id: NodeId) {
        let bytes = json.as_bytes();
        let dst_socket_addr = self.socket_addrs.get(&receiver_id).expect("Could not find receiver among the socket addresses");
        self.socket.send_to(bytes, dst_socket_addr).expect("Could not send on the socket");
    }
}