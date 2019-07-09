
use crate::abd_node::AbdNode;
use crate::communicator::Communicator;

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

use crate::settings::SETTINGS;

struct Mediator<AbdV> {
    communicator: Communicator,
    abd_node: AbdNode<AbdV>
}

impl<AbdV: Default + Serialize + DeserializeOwned + Debug + Clone> Mediator<AbdV> {
    pub fn new() -> Mediator<AbdV> {
        let node_id = SETTINGS.node_id;
        let socket_addrs = SETTINGS.socket_addrs.clone();

        let mut node_ids = HashSet::new();
        for &node_id in socket_addrs.keys() {
            node_ids.insert(node_id);
        }


        Mediator {
            communicator: Communicator::new(node_id, socket_addrs).unwrap(),
            abd_node: AbdNode::new(node_id, node_ids)
        }
    }
}