use std::borrow::Cow;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

use commons::run_result::RunResult;
use commons::types::{Int, NodeId};

use crate::abd_node::AbdNode;
use crate::mediator::Mediator;
use crate::messages::{self, Message, WriteAckMessage, WriteMessage};
use crate::responsible_cell::ResponsibleCell;

mod start_values;
mod write;


struct MockMediator {
    node_id: NodeId,
    node_ids: HashSet<NodeId>,

    run_result: Mutex<RunResult>,
    abd_node: ResponsibleCell<Option<AbdNode<MockMediator, String>>>,
    
    sent_write_messages: Mutex<Vec<WriteMessage<String>>>,
    write_message_receivers: Mutex<HashSet<NodeId>>,
    sent_write_ack_messages: Mutex<Vec<WriteAckMessage>>,
    write_ack_message_receivers: Mutex<HashSet<NodeId>>,
}

impl MockMediator {
    pub fn new(
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    ) -> Arc<MockMediator> {
        let mediator = MockMediator {
            node_id: node_id,
            node_ids: node_ids,
            run_result: Mutex::new(RunResult::new()),
            abd_node: ResponsibleCell::new(None),
            sent_write_messages: Mutex::new(Vec::new()),
            write_message_receivers: Mutex::new(HashSet::new()),
            sent_write_ack_messages: Mutex::new(Vec::new()),
            write_ack_message_receivers: Mutex::new(HashSet::new()),
        };
        let mediator = Arc::new(mediator);
        let abd_node: AbdNode<MockMediator, String> =
            AbdNode::new(Arc::downgrade(&mediator));
        *mediator.abd_node.get_mut() = Some(abd_node);

        mediator
    }

    pub fn abd_node(&self) -> &AbdNode<MockMediator, String> {
        self.abd_node.get().as_ref().unwrap()
    }
}

impl Mediator for MockMediator {
    fn send_json_to(&self, json: &str, receiver: NodeId) {
        if messages::json_is_write_message(json) {
            self.sent_write_messages
                .lock()
                .unwrap()
                .push(
                    serde_json::from_str(json)
                        .expect("Could not derserialize a write message."),
                );
            self.write_message_receivers
                .lock()
                .unwrap()
                .insert(receiver);
        } else if messages::json_is_write_ack_message(json) {
            println!("Hej");
            self.sent_write_ack_messages
                .lock()
                .unwrap()
                .push(serde_json::from_str(json).expect(
                    "Could not derserialize a write ack message.",
                ));
            self.write_ack_message_receivers
                .lock()
                .unwrap()
                .insert(receiver);
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

    fn read(&self) -> String {
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

fn create_mediator() -> Arc<MockMediator> {
    let node_id = 1;
    let node_ids = node_ids_for_tests();
    MockMediator::new(node_id, node_ids.clone())
}
