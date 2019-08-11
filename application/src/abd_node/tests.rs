
#[cfg(test)]

use super::*;

use std::str;
use std::sync::{Arc, Mutex, MutexGuard, Condvar, Weak};
use std::collections::HashSet;
use std::borrow::Cow;
use std::fmt::Debug;
use std::time::Duration;
use std::iter::FromIterator;
use std::thread::{self, JoinHandle};

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
use crate::responsible_cell::ResponsibleCell;
use crate::abd_node::AbdNode;


struct MockMediator {
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    sent_write_messages: Mutex<Vec<WriteMessage<'static, String>>>,
    run_result: Mutex<RunResult>,
    abd_node: ResponsibleCell<Option<AbdNode<MockMediator, String>>>,
}

impl MockMediator {
    pub fn new(node_id: NodeId, node_ids: HashSet<NodeId>) -> Arc<MockMediator> {
        let mediator = MockMediator {
            node_id: node_id,
            node_ids: node_ids,
            sent_write_messages: Mutex::new(Vec::new()),
            run_result: Mutex::new(RunResult::new()),
            abd_node: ResponsibleCell::new(None)
        };
        let mediator = Arc::new(mediator);
        let abd_node: AbdNode<MockMediator, String> = AbdNode::new(Arc::downgrade(&mediator));
        *mediator.abd_node.get_mut() = Some(abd_node);

        mediator
    }

    pub fn abd_node(&self) -> &AbdNode<MockMediator, String> {
        self.abd_node.get().as_ref().unwrap()
    }
}

impl Mediator for MockMediator{
    fn send_json_to(&self, json: &str, receiver: NodeId) {
        if messages::json_is_write_message(json) {
            self.sent_write_messages.lock().expect("Could not lock sent write messages.").push(serde_json::from_str(json).expect("Could not derserialize a write message."));
        }
    }

    fn broadcast_json(&self, json: &str) {
        for &node_id in &self.node_ids {
            self.send_json_to(json, node_id);
        }
    }
    
    fn json_received(&self, json: &str) {
        self.abd_node().json_received(json);
    }
    

    fn node_id(&self) -> NodeId {
        self.node_id
    }
    
    fn node_ids(&self) -> &HashSet<NodeId> {
        &self.node_ids
    }
    
    fn number_of_nodes(&self) -> Int {
        self.node_ids.len() as Int
    }
    

    fn run_result(&self) -> MutexGuard<RunResult> {
        self.run_result.lock().unwrap()
    }
    

    fn write(&self, message: String) {
        self.abd_node().write(message);
    }
    
    fn read(&self, node_id: NodeId) -> String {
        panic!("Unused");
    }
    
    fn read_all(&self) -> RegisterArray<String> {
        panic!("Unused");
    }

    fn record_evaluation_info(&self) -> bool {
        true
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
        let mediator = Arc::new(MockMediator::new(1, node_ids));
        let abd_node: AbdNode<MockMediator, String> = AbdNode::new(Arc::downgrade(&mediator));

        assert_eq!(abd_node.number_of_nodes_in_a_majority(), number_of_nodes_in_a_majority[i]);
    }
}


#[test]
fn test_that_write_sends_correct_messages() {
    let mediator = create_mediator();
    let write_thread_handle = perform_single_write_on_background_thread(&mediator);
    wait_until_local_register_array_is_written(&mediator);    
    send_write_ack_message_from_all_nodes(&mediator);
    write_thread_handle.join().unwrap();
    check_that_sent_write_messages_are_the_expected_form(&mediator);
}

fn create_mediator() -> Arc<MockMediator> {
    let node_id = 1;
    let node_ids = node_ids_for_tests();
    MockMediator::new(node_id, node_ids.clone())
}

fn perform_single_write_on_background_thread(mediator: &Arc<MockMediator>) -> JoinHandle<()> {
    let mediator_for_write_thread = Arc::clone(&mediator);
    thread::spawn(move || {
        mediator_for_write_thread.write("Rust".to_string());
    })
}

fn wait_until_local_register_array_is_written(mediator: &Arc<MockMediator>) {
    while mediator.abd_node().reg.lock().unwrap().get(mediator.node_id).ts == timestamp::default_timestamp() {

    }
}

fn send_write_ack_message_from_all_nodes(mediator: &Arc<MockMediator>) {
    for &node_id in mediator.node_ids.iter() {
        let json;

        {
            let reg_array = &mediator.abd_node().reg.lock().unwrap();
            let write_ack_message = WriteAckMessage {
                sender: node_id,
                register_array: Cow::Borrowed(reg_array)
            };
            
            json = serde_json::to_string(&write_ack_message).unwrap();
        }

        mediator.json_received(&json);
    }
}

fn check_that_sent_write_messages_are_the_expected_form(mediator: &Arc<MockMediator>) {
    let reg_array = mediator.abd_node().reg.lock().unwrap();
    let expected_write_message = WriteMessage {
        sender: mediator.node_id,
        register_array: Cow::Borrowed(&reg_array)
    };

    for write_message in mediator.sent_write_messages.lock().unwrap().iter() {
        assert_eq!(*write_message, expected_write_message);
    }
}