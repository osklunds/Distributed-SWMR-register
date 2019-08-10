
use std::sync::{Arc, Mutex, MutexGuard};

use commons::run_result::RunResult;
use commons::types::NodeId;

//use crate::terminal_output::printlnu;
use crate::settings::SETTINGS;
use crate::responsible_cell::ResponsibleCell;
use crate::abd_node::AbdNode;
use crate::communicator::Communicator;
use crate::data_types::register_array::RegisterArray;
use crate::messages;


pub struct Mediator {
    communicator: ResponsibleCell<Option<Arc<Communicator>>>,
    abd_node: ResponsibleCell<Option<AbdNode<String>>>,

    run_result: Mutex<RunResult>
}

impl Mediator {
    pub fn new() -> Arc<Mediator> {
        let mediator = Mediator {
            communicator: ResponsibleCell::new(None),
            abd_node: ResponsibleCell::new(None),
            run_result: Mutex::new(RunResult::new())
        };
        let mediator = Arc::new(mediator);

        let node_id = SETTINGS.node_id();
        let socket_addrs = SETTINGS.socket_addrs().clone();
        let own_socket_addr = socket_addrs.get(&node_id).expect("Could not find own socket addres.");

        let communicator = Communicator::new(*own_socket_addr, socket_addrs, Arc::downgrade(&mediator));
        let abd_node: AbdNode<String> = AbdNode::new(Arc::downgrade(&mediator));

        *mediator.communicator.get_mut() = Some(communicator);
        *mediator.abd_node.get_mut() = Some(abd_node);

        mediator
    }
    

    // Modules

    fn abd_node(&self) -> &AbdNode<String> {
        self.abd_node.get().as_ref().expect("AbdNode not set on Mediator.")
    }
    
    fn communicator(&self) -> &Communicator {
        self.communicator.get().as_ref().expect("Communicator not set on Mediator.")
    }


    // Communicator

    pub fn send_json_to(&self, json: &str, receiver: NodeId) {
        self.communicator().send_json_to(json, receiver);
    }

    pub fn broadcast_json(&self, json: &str) {
        for &node_id in SETTINGS.socket_addrs().keys() {
            self.send_json_to(json, node_id);
        }
    }

    pub fn json_received(&self, json: &str) {
        self.abd_node().json_received(json);
    }


    // Evaluation

    pub fn run_result(&self) -> MutexGuard<RunResult> {
        self.run_result.lock().unwrap()
    }


    // Abd Node
        
    pub fn write(&self, message: String) {
        self.abd_node().write(message);
    }

    
    pub fn read(&self, node_id: NodeId) -> String {
        self.abd_node().read(node_id)
    }

    
    pub fn read_all(&self) -> RegisterArray<String> {
        self.abd_node().read_all()
    }
}