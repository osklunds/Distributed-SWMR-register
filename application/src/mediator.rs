
use std::sync::{Arc, Mutex, MutexGuard};
use std::marker::{Send, Sync};
use std::collections::HashSet;

use commons::run_result::RunResult;
use commons::types::NodeId;

//use crate::terminal_output::printlnu;
use crate::settings::SETTINGS;
use crate::responsible_cell::ResponsibleCell;
use crate::abd_node::AbdNode;
use crate::communicator::Communicator;
use crate::data_types::register_array::RegisterArray;
use crate::configuration_manager::ConfigurationManager;
use crate::messages;


pub trait Mediator {
    // Communicator

    fn send_json_to(&self, json: &str, receiver: NodeId);
    fn broadcast_json(&self, json: &str);
    fn json_received(&self, json: &str);

    // Configuration manager

    fn node_id(&self) -> NodeId;
    fn node_ids(&self) -> &HashSet<NodeId>;


    // Evaluation

    fn run_result(&self) -> MutexGuard<RunResult>;


    // Abd Node
        
    fn write(&self, message: String);
    fn read(&self, node_id: NodeId) -> String;
    fn read_all(&self) -> RegisterArray<String>;
}


pub trait Med: Mediator + Send + Sync + 'static {}
impl<T: Mediator + Send + Sync + 'static> Med for T {}


pub struct MediatorImpl {
    communicator: ResponsibleCell<Option<Arc<Communicator<MediatorImpl>>>>,
    configuration_manager: ConfigurationManager,
    run_result: Mutex<RunResult>,

    abd_node: ResponsibleCell<Option<AbdNode<String, MediatorImpl>>>,
}

impl MediatorImpl {
    pub fn new() -> Arc<MediatorImpl> {
        let node_id = SETTINGS.node_id();
        let socket_addrs = SETTINGS.socket_addrs().clone();
        let node_ids = socket_addrs.keys().map(|node_id| *node_id).collect();

        let mediator = MediatorImpl {
            communicator: ResponsibleCell::new(None),
            configuration_manager: ConfigurationManager::new(node_id, node_ids),
            run_result: Mutex::new(RunResult::new()),
            abd_node: ResponsibleCell::new(None)
        };
        let mediator: Arc<MediatorImpl> = Arc::new(mediator);

        let own_socket_addr = socket_addrs.get(&node_id).expect("Could not find own socket addres.");

        let communicator = Communicator::new(*own_socket_addr, socket_addrs, Arc::downgrade(&mediator));
        let abd_node = AbdNode::new(Arc::downgrade(&mediator));

        *mediator.communicator.get_mut() = Some(communicator);
        *mediator.abd_node.get_mut() = Some(abd_node);

        mediator
    }
    

    // Modules

    fn communicator(&self) -> &Communicator<MediatorImpl> {
        self.communicator.get().as_ref().expect("Communicator not set on MediatorImpl.")
    }

    fn abd_node(&self) -> &AbdNode<String, MediatorImpl> {
        self.abd_node.get().as_ref().expect("AbdNode not set on MediatorImpl.")
    }

    fn configuration_manager(&self) -> &ConfigurationManager {
        &self.configuration_manager
    }
}

impl Mediator for MediatorImpl {
    // Communicator

    fn send_json_to(&self, json: &str, receiver: NodeId) {
        self.communicator().send_json_to(json, receiver);
    }

    fn broadcast_json(&self, json: &str) {
        for &node_id in SETTINGS.socket_addrs().keys() {
            self.send_json_to(json, node_id);
        }
    }

    fn json_received(&self, json: &str) {
        self.abd_node().json_received(json);
    }

    // Configuration manager

    fn node_id(&self) -> NodeId {
        self.configuration_manager().node_id()
    }

    fn node_ids(&self) -> &HashSet<NodeId> {
        self.configuration_manager().node_ids()
    }


    // Evaluation

    fn run_result(&self) -> MutexGuard<RunResult> {
        self.run_result.lock().unwrap()
    }


    // Abd Node
        
    fn write(&self, message: String) {
        self.abd_node().write(message);
    }

    
    fn read(&self, node_id: NodeId) -> String {
        self.abd_node().read(node_id)
    }

    
    fn read_all(&self) -> RegisterArray<String> {
        self.abd_node().read_all()
    }
}