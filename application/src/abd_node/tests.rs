
#[cfg(test)]

use super::*;

use std::str;
use std::sync::{Arc, Mutex, MutexGuard, Condvar, Weak};
use std::collections::HashSet;
use std::borrow::Cow;
use std::fmt::Debug;
use std::time::Duration;
use std::iter::FromIterator;

use serde::Serialize;
use serde::de::DeserializeOwned;

use commons::types::{NodeId, Int};
use commons::run_result::RunResult;

use crate::terminal_output::printlnu;
use crate::settings::SETTINGS;
use crate::data_types::timestamp::{self, Timestamp};
use crate::data_types::register_array::*;
use crate::data_types::register::{self, Register};
use crate::messages::{self, Message, WriteMessage, WriteAckMessage, ReadMessage, ReadAckMessage};
use crate::mediator::Mediator;


struct MockMediator {
    node_ids: HashSet<NodeId>
}

impl MockMediator {
    pub fn new(node_ids: HashSet<NodeId>) -> MockMediator {
        MockMediator {
            node_ids: node_ids
        }
    }
}

impl Mediator for MockMediator {
    fn send_json_to(&self, json: &str, receiver: NodeId) {

    }

    fn broadcast_json(&self, json: &str) {

    }
    
    fn json_received(&self, json: &str) {

    }
    

    fn node_id(&self) -> NodeId {
        panic!("Unused");
    }
    
    fn node_ids(&self) -> &HashSet<NodeId> {
        &self.node_ids
    }
    
    fn number_of_nodes(&self) -> Int {
        self.node_ids.len() as Int
    }
    

    fn run_result(&self) -> MutexGuard<RunResult> {
        panic!("Unused");
    }
    

    fn write(&self, message: String) {

    }
    
    fn read(&self, node_id: NodeId) -> String {
        panic!("Unused");
    }
    
    fn read_all(&self) -> RegisterArray<String> {
        panic!("Unused");
    }
}


fn node_ids_for_tests() -> HashSet<NodeId> {
    let mut node_ids = HashSet::new();
    node_ids.insert(1);
    node_ids.insert(2);
    node_ids.insert(3);
    node_ids.insert(4);
    node_ids
}

#[test]
fn test_number_of_nodes_in_a_majority() {
    let node_ids = vec![vec![1],
                        vec![1,2],
                        vec![1,2,3],
                        vec![1,2,3,4],
                        vec![1,2,3,4,5],
                        vec![1,2,3,4,5,6],
                        vec![1,2,3,4,5,6,7]];
    let number_of_nodes_in_a_majority = vec![1,2,2,3,3,4,4];

    for i in 0..node_ids.len() {
        let node_ids = node_ids[i].clone();
        let node_ids = HashSet::from_iter(node_ids);
        let mediator = Arc::new(MockMediator::new(node_ids));
        let abd_node: AbdNode<MockMediator, String> = AbdNode::new(Arc::downgrade(&mediator));

        assert_eq!(abd_node.number_of_nodes_in_a_majority(), number_of_nodes_in_a_majority[i]);
    }
}