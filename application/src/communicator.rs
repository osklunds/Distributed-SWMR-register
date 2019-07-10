
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
use crate::abd_node::AbdNode;
use crate::mediator::Mediator;

pub struct Communicator {
    id: Arc<NodeId>,
    socket: Arc<UdpSocket>,
    socket_addrs: Arc<HashMap<NodeId, SocketAddr>>,
    pub mediator: Option<Arc<Mediator>>
}

impl Communicator {
    pub fn new(node_id: NodeId, socket_addrs: HashMap<NodeId, SocketAddr>) -> io::Result<Communicator> {
        let my_socket_addr = socket_addrs.get(&node_id).expect("My node id was not included among the socket addresses.");
        let socket = UdpSocket::bind(my_socket_addr)?;

        Ok(Communicator {
            id: Arc::new(node_id),
            socket: Arc::new(socket),
            socket_addrs: Arc::new(socket_addrs),
            mediator: None
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

    fn mediator(&self) -> &Arc<Mediator> {
        if let Some(med) = &self.mediator {
            return med;
        } else {
            panic!("Mediator not set");
        }
    }
}