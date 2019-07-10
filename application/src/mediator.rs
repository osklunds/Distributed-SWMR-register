
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

pub struct Mediator {
    communicator: Option<Arc<Communicator>>,
    abd_node: Option<Arc<AbdNode<String>>>
}

impl Mediator {
    pub fn new() -> Arc<Mediator> {
        let node_id = SETTINGS.node_id;
        let socket_addrs = SETTINGS.socket_addrs.clone();

        let mut node_ids = HashSet::new();
        for &node_id in socket_addrs.keys() {
            node_ids.insert(node_id);
        }

        let mut mediator = Mediator {
            communicator: None,
            abd_node: None,
        };


        let mut communicator = Communicator::new(node_id, socket_addrs).unwrap();

        let mut abd_node = AbdNode::new(node_id, node_ids);

        let mediator_raw = &mut mediator as *mut Mediator;
        let communicator_raw = &mut communicator as *mut Communicator;
        let abd_node_raw = &mut abd_node as *mut AbdNode<String>;

        let mediator = Arc::new(mediator);

        unsafe {
            (*mediator_raw).communicator = Some(Arc::new(communicator));
            (*mediator_raw).abd_node = Some(Arc::new(abd_node));

            (*communicator_raw).mediator = Some(Arc::clone(&mediator));
            (*abd_node_raw).mediator = Some(Arc::clone(&mediator));
        }

        mediator
    }

    /*
    pub fn setup_communicator(mediator: Arc<Mediator>) {
        mediator.communicator.mediator = Some(mediator);

        let recv_thread_node = Arc::clone(&mediator.communicator);
        let recv_thread_handle = thread::spawn(move || {
            recv_thread_node.recv_loop(mediator);
        });
    }
    */

    pub fn send_json_to(&self, json: &str, receiver: NodeId) {

    }

    pub fn json_received(&self, json: &str) {

    }
}