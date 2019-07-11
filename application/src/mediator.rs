
use std::sync::Arc;
use std::thread;
use std::collections::HashSet;

//use crate::terminal_output::printlnu;
use crate::settings::{SETTINGS, NodeId};
use crate::responsible_cell::ResponsibleCell;
use crate::abd_node::AbdNode;
use crate::communicator::Communicator;


pub struct Mediator {
    communicator: ResponsibleCell<Option<Communicator>>,
    abd_node: ResponsibleCell<Option<AbdNode<String>>>
}

impl Mediator {
    pub fn new() -> Arc<Mediator> {
        let node_id = SETTINGS.node_id();
        let socket_addrs = SETTINGS.socket_addrs().clone();

        let mut node_ids = HashSet::new();
        for &node_id in socket_addrs.keys() {
            node_ids.insert(node_id);
        }

        let mediator = Mediator {
            communicator: ResponsibleCell::new(None),
            abd_node: ResponsibleCell::new(None),
        };
        let mediator = Arc::new(mediator);

        let communicator = Communicator::new(node_id, socket_addrs, Arc::downgrade(&mediator));
        let abd_node: AbdNode<String> = AbdNode::new(Arc::downgrade(&mediator));

        *mediator.communicator.get_mut() = Some(communicator);
        *mediator.abd_node.get_mut() = Some(abd_node);

        Self::start_recv_thread(Arc::clone(&mediator));

        mediator
    }

    
    fn start_recv_thread(mediator: Arc<Mediator>) {
        let recv_thread_handle = thread::spawn(move || {
            mediator.communicator().recv_loop();
         });
    }
    
    fn abd_node(&self) -> &AbdNode<String> {
        self.abd_node.get().as_ref().unwrap()
    }

    
    fn communicator(&self) -> &Communicator {
        self.communicator.get().as_ref().unwrap()
    }

    pub fn send_json_to(&self, json: &str, receiver: NodeId) {
        self.communicator().send_json_to(json, receiver);
    }

    pub fn broadcast_json(&self, json: &str) {
        self.communicator().broadcast_json(json);
    }

    pub fn json_received(&self, json: &str) {
        self.abd_node().json_received(json);
    }

    pub fn write(&self, message: String) {
        self.abd_node().write(message);
    }

    pub fn node_id(&self) -> NodeId {
        self.communicator().id
    }
}