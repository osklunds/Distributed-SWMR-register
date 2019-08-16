
use std::str;
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashSet;
use std::borrow::Cow;
use std::thread::{self, JoinHandle};
use std::iter::FromIterator;

use commons::types::{NodeId, Int};
use commons::run_result::RunResult;

use crate::data_types::timestamp;
use crate::data_types::register::Register;
use crate::data_types::register_array::*;
use crate::messages::{self, WriteMessage, WriteAckMessage};
use crate::mediator::Mediator;
use crate::responsible_cell::ResponsibleCell;
use crate::abd_node::AbdNode;


struct MockMediator {
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    run_result: Mutex<RunResult>,
    abd_node: ResponsibleCell<Option<AbdNode<MockMediator, String>>>,
    sent_write_messages: Mutex<Vec<WriteMessage<'static, String>>>,
    write_message_receivers: Mutex<HashSet<NodeId>>,
    sent_write_ack_messages: Mutex<Vec<WriteAckMessage<'static, String>>>,
    write_ack_message_receivers: Mutex<HashSet<NodeId>>
}

impl MockMediator {
    pub fn new(node_id: NodeId, node_ids: HashSet<NodeId>) -> Arc<MockMediator> {
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
            self.sent_write_messages.lock()
                .expect("Could not lock sent write messages.")
                .push(serde_json::from_str(json)
                .expect("Could not derserialize a write message."));
            self.write_message_receivers.lock().expect("Could not lock write message receivers.").insert(receiver);
        } else if messages::json_is_write_ack_message(json) {
            self.sent_write_ack_messages.lock()
                .expect("Could not lock sent write ack messages.")
                .push(serde_json::from_str(json)
                .expect("Could not derserialize a write ack message."));
            self.write_ack_message_receivers.lock().expect("Could not lock write ack message receivers.").insert(receiver);
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
    
    fn read(&self, _node_id: NodeId) -> String {
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


fn create_mediator_perform_write_and_ack() -> Arc<MockMediator> {
    let mediator = create_mediator();
    let write_thread_handle = perform_single_write_on_background_thread(&mediator);
    wait_until_local_register_array_is_written(&mediator);    
    send_write_ack_message_from_node_ids(&mediator, node_ids_for_tests());
    write_thread_handle.join().unwrap();
    mediator
}

fn create_mediator() -> Arc<MockMediator> {
    let node_id = 1;
    let node_ids = node_ids_for_tests();
    MockMediator::new(node_id, node_ids.clone())
}

fn perform_single_write_on_background_thread(mediator: &Arc<MockMediator>) -> JoinHandle<()> {
    let mediator_for_write_thread = Arc::clone(&mediator);
    thread::spawn(move || {
        mediator_for_write_thread.write(value_for_writes());
    })
}

fn value_for_writes() -> String {
    format!("Rust")
}

fn wait_until_local_register_array_is_written(mediator: &Arc<MockMediator>) {
    while mediator.abd_node().reg.lock().unwrap().get(mediator.node_id).ts == timestamp::default_timestamp() {

    }
}


mod termination {
    use super::*;

    // If writes don't terminates, neither will the tests.
    #[test]
    fn test_that_write_terminates() {
        let mediator = create_mediator();
        let write_thread_handle = perform_single_write_on_background_thread(&mediator);
        wait_until_local_register_array_is_written(&mediator);

        send_write_ack_message_from_node_ids(&mediator, node_ids_for_tests());

        write_thread_handle.join().unwrap();
    }

    #[test]
    fn test_that_write_terminates_even_if_not_all_nodes_ack() {
        let mediator = create_mediator();
        let write_thread_handle = perform_single_write_on_background_thread(&mediator);
        wait_until_local_register_array_is_written(&mediator);

        let mut node_ids = HashSet::new();
        node_ids.insert(1);  
        node_ids.insert(2);
        node_ids.insert(3);
        send_write_ack_message_from_node_ids(&mediator, node_ids);

        write_thread_handle.join().unwrap();
    }

    #[test]
    fn test_that_write_does_not_terminate_without_acks() {
        let mediator = create_mediator();
        perform_single_write_on_background_thread(&mediator);
        wait_until_local_register_array_is_written(&mediator);

        check_that_write_fails(&mediator);
    }

    #[test]
    fn test_that_write_does_not_terminate_without_acks_from_majority() {
        let mediator = create_mediator();
        perform_single_write_on_background_thread(&mediator);
        wait_until_local_register_array_is_written(&mediator);

        let mut node_ids = HashSet::new();
        node_ids.insert(2);
        node_ids.insert(3);
        send_write_ack_message_from_node_ids(&mediator, node_ids);

        check_that_write_fails(&mediator);
    }

    fn check_that_write_fails(mediator: &Arc<MockMediator>) {
        // If this test terminates, it means that retransmissions had
        // to be done, because no write acks received.
        while mediator.sent_write_messages.lock()
            .expect("Could not lock sent write messages.")
            .len() <= node_ids_for_tests().len() * 3 {
        }

        let register_array_being_written = mediator.abd_node().register_array_being_written.lock()
            .expect("Could not lock register array being written.");

        assert_ne!(*register_array_being_written, None);
    }
}


fn send_write_ack_message_from_node_ids(mediator: &Arc<MockMediator>, node_ids: HashSet<NodeId>) {
    for &node_id in node_ids.iter() {
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


mod message_sending {
    use super::*;

    #[test]
    fn test_that_write_sends_correct_write_messages() {
        let mediator = create_mediator_perform_write_and_ack();
        check_that_sent_write_messages_are_the_expected_form(&mediator);
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

    #[test]
    fn test_that_write_sends_write_messages_to_the_correct_nodes() {
        let mediator = create_mediator_perform_write_and_ack();
        check_that_write_messages_are_sent_to_the_correct_nodes(&mediator);
    }

    fn check_that_write_messages_are_sent_to_the_correct_nodes(mediator: &Arc<MockMediator>) {
        assert_eq!(*mediator.write_message_receivers.lock()
                .expect("Could not lock write message receivers."),
                mediator.node_ids);
    }

    #[test]
    fn test_that_write_sends_correct_write_ack_messages() {
        let mediator = create_mediator_perform_write_and_ack();
        check_that_sent_write_ack_messages_are_the_expected_form(&mediator);
    }

    fn check_that_sent_write_ack_messages_are_the_expected_form(mediator: &Arc<MockMediator>) {
        let reg_array = mediator.abd_node().reg.lock().unwrap();
        let expected_write_ack_message = WriteAckMessage {
            sender: mediator.node_id,
            register_array: Cow::Borrowed(&reg_array)
        };

        for write_ack_message in mediator.sent_write_ack_messages.lock().unwrap().iter() {
            assert_eq!(*write_ack_message, expected_write_ack_message);
        }
    }
}


mod start_values {
    use super::*;

    #[test]
    fn test_that_register_array_is_empty_at_start() {
        let mediator = create_mediator();
        let register = Register::new(timestamp::default_timestamp(), String::default());

        for &node_id in mediator.node_ids.iter() {
            assert_eq!(mediator.abd_node().reg.lock()
                        .expect("Could not lock register array")
                        .get(node_id),
                       &register);
        }
    }

    #[test]
    fn test_that_ts_is_0_at_start() {
        let mediator = create_mediator();
        assert_eq!(*mediator.abd_node().ts.lock().expect("Could not lock ts."), 
            0);
    }

    #[test]
    fn test_that_register_array_being_written_is_none_at_start() {
        let mediator = create_mediator();
        assert_eq!(*mediator.abd_node()
            .register_array_being_written.lock()
            .expect("Could not lock register array."), 
            None);
    }

    #[test]
    fn test_that_acking_processors_for_write_is_empty_at_start() {
        let mediator = create_mediator();
        assert!(mediator.abd_node()
            .acking_processors_for_write.lock()
            .expect("Could not lock register array.").is_empty());
    }
}


mod variable_changes {
    use super::*;

    #[test]
    fn test_that_own_register_array_is_updated_correctly_on_write() {
        let mediator = create_mediator();
        perform_single_write_on_background_thread(&mediator);
        wait_until_local_register_array_is_written(&mediator);
        let own_register_array = mediator.abd_node().reg.lock().unwrap();
        let mut expected_register_array = RegisterArray::new(&mediator.node_ids);
        let register = Register::new(1, value_for_writes());
        expected_register_array.set(mediator.node_id, register);
        assert_eq!(*own_register_array, expected_register_array);
    }



    #[test]
    fn test_that_a_write_message_updates_own_register_array() {
        let mediator = create_mediator();
        let reg_array = send_a_register_array_in_a_write_message(&mediator);
        let reg_array_abd_node = mediator.abd_node().reg.lock()
            .expect("Could not lock register array.");

        assert_eq!(*reg_array_abd_node, reg_array);
    }

    fn send_a_register_array_in_a_write_message(mediator: &Arc<MockMediator>) -> RegisterArray<String> {
        let mut reg_array = mediator.abd_node().reg.lock()
            .expect("Could not lock register array.").clone();
        reg_array.set(2, Register::new(7, "Haskell".to_string()));
        reg_array.set(3, Register::new(10, "Idris".to_string()));

        let write_message = WriteMessage {
            sender: 2,
            register_array: Cow::Owned(reg_array.clone())
        };
        let json = serde_json::to_string(&write_message)
            .expect("Could not serialize a write message");
        
        mediator.json_received(&json);
        reg_array
    }

    #[test]
    fn test_that_a_write_message_does_not_change_register_being_written() {
        let mediator = create_mediator();
        send_a_register_array_in_a_write_message(&mediator);
        assert_eq!(*mediator.abd_node().register_array_being_written.lock()
            .expect("Could not lock register array being written"), None);
    }

    #[test]
    fn test_that_a_write_ack_message_updates_own_register_array() {
        let mediator = create_mediator();
        let reg_array = send_a_register_array_in_a_write_ack_message(&mediator);
        let reg_array_abd_node = mediator.abd_node().reg.lock()
            .expect("Could not lock register array.");

        assert_eq!(*reg_array_abd_node, reg_array);
    }

    fn send_a_register_array_in_a_write_ack_message(mediator: &Arc<MockMediator>) -> RegisterArray<String> {
        let mut reg_array = mediator.abd_node().reg.lock()
            .expect("Could not lock register array.").clone();
        reg_array.set(2, Register::new(7, "Haskell".to_string()));
        reg_array.set(3, Register::new(10, "Idris".to_string()));

        let write_ack_message = WriteAckMessage {
            sender: 2,
            register_array: Cow::Owned(reg_array.clone())
        };
        let json = serde_json::to_string(&write_ack_message)
            .expect("Could not serialize a write ack message");
        
        mediator.json_received(&json);
        reg_array
    }

    #[test]
    fn test_that_a_write_ack_message_does_not_change_register_array_being_written_when_there_is_no_write() {
        let mediator = create_mediator();
        send_a_register_array_in_a_write_message(&mediator);
        let register_array_being_written = mediator.abd_node().register_array_being_written.lock()
            .expect("Could not lock register array being written.");

        assert_eq!(*register_array_being_written, None);
    }

    #[test]
    fn test_that_a_write_ack_message_does_not_change_acking_processors_for_write_when_there_is_no_write() {
        let mediator = create_mediator();
        send_a_register_array_in_a_write_message(&mediator);
        let acking_processors_for_write = mediator.abd_node().acking_processors_for_write.lock()
            .expect("Could not lock acking_processors_for_write.");

        assert!(acking_processors_for_write.is_empty());
    }
}
