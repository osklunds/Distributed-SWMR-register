
use std::sync::{Arc, Mutex, MutexGuard};

//use crate::terminal_output::printlnu;
use crate::settings::{SETTINGS, NodeId};
use crate::responsible_cell::ResponsibleCell;
use crate::abd_node::AbdNode;
use crate::communicator::Communicator;
use crate::register::Register;
use crate::run_result::RunResult;
use crate::messages;


pub struct Mediator {
    communicator: ResponsibleCell<Option<Arc<Communicator>>>,
    abd_node: ResponsibleCell<Option<AbdNode<String>>>,

    run_result: Mutex<RunResult>
}

impl Mediator {
    #[allow(dead_code)]
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
    
    fn abd_node(&self) -> &AbdNode<String> {
        self.abd_node.get().as_ref().expect("AbdNode not set on Mediator.")
    }
    
    fn communicator(&self) -> &Communicator {
        self.communicator.get().as_ref().expect("Communicator not set on Mediator.")
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

    #[allow(dead_code)]
    pub fn write(&self, message: String) {
        self.abd_node().write(message);
    }

    #[allow(dead_code)]
    pub fn read(&self, node_id: NodeId) -> String {
        self.abd_node().read(node_id)
    }

    #[allow(dead_code)]
    pub fn read_all(&self) -> Register<String> {
        self.abd_node().read_all()
    }

    pub fn run_result(&self) -> MutexGuard<RunResult> {
        self.run_result.lock().unwrap()
    }
}