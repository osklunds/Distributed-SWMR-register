use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, Weak};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use commons::types::{Int, NodeId};

use crate::data_types::register::Register;
use crate::data_types::register_array::*;
use crate::data_types::timestamp::Timestamp;
use crate::mediator::Med;
use crate::messages::*;
use crate::quorum::Quorum;
use crate::terminal_output::printlnu;

#[cfg(test)]
pub mod tests;

pub struct AbdNode<M, V> {
    mediator: Weak<M>,

    timestamp: Mutex<Timestamp>,
    value: Mutex<V>,

    write_quorum: Quorum,
    read1_quorum: Quorum,
    read2_quorum: Quorum,
}

pub trait Value: Default + Serialize + DeserializeOwned + Debug + Clone {}
impl <V: Default + Serialize + DeserializeOwned + Debug + Clone> Value for V {}

impl<
        V: Value,
        M: Med,
    > AbdNode<M, V>
{
    pub fn new(mediator: Weak<M>) -> AbdNode<M, V> {
        let mediator_upgraded = mediator
            .upgrade()
            .expect("Error upgrading mediator in AbdNode constructor");
        let node_ids = mediator_upgraded.node_ids();

        AbdNode {
            mediator: mediator,
            
            timestamp: Mutex::new(0),
            value: Mutex::new(V::default()),

            write_quorum: Quorum::new(),
            read1_quorum: Quorum::new(),
            read2_quorum: Quorum::new(),
        }
    }
    /*

    #[allow(dead_code)]
    pub fn write(&self, value: V) {
        self.inner_write(value);

        if cfg!(debug_assertions) {
            let acking_processors_for_write =
                self.acking_processors_for_write.lock().unwrap();
            let register_array_being_written =
                self.register_array_being_written.lock().unwrap();
            assert!(acking_processors_for_write.is_empty());
            assert!(register_array_being_written.is_none());
        }

        if self.mediator().record_evaluation_info() {
            self.mediator().run_result().write_ops += 1;
        }
    }

    fn inner_write(&self, value: V) {
        let register_array =
            self.acquire_register_array_and_update_it_with_value(value);
        self.clone_register_array_to_register_array_being_written(
            &register_array,
        );
        let json_write_message = self
            .construct_json_write_message_and_release_register_array(
                register_array,
            );

        self.broadcast_json_write_message_until_majority_has_acked(
            &json_write_message,
        );
    }

    fn acquire_register_array_and_update_it_with_value(
        &self,
        value: V,
    ) -> MutexGuard<RegisterArray<V>> {
        let mut ts = self.ts.lock().unwrap();
        let mut reg = self.reg.lock().unwrap();

        *ts += 1;
        reg.set(self.mediator().node_id(), Register::new(*ts, value));
        reg
    }

    fn clone_register_array_to_register_array_being_written(
        &self,
        register_array: &RegisterArray<V>,
    ) {
        let mut register_array_being_written =
            self.register_array_being_written.lock().unwrap();

        if cfg!(debug_assertions) {
            assert_eq!(*register_array_being_written, None);
        }

        *register_array_being_written = Some(register_array.clone());
    }

    fn construct_json_write_message_and_release_register_array(
        &self,
        register_array: MutexGuard<RegisterArray<V>>,
    ) -> String {
        let write_message = WriteMessage {
            sender: self.mediator().node_id(),
            register_array: Cow::Borrowed(&register_array),
        };

        serde_json::to_string(&write_message)
            .expect("Could not serialize a message")
    }

    fn jsonify_message<Me: Message>(&self, message: &Me) -> String {
        serde_json::to_string(message)
            .expect("Could not serialize a message")
    }

    fn broadcast_json_write_message_until_majority_has_acked(
        &self,
        json_write_message: &str,
    ) {
        self.broadcast_json_message(&json_write_message);
        if self.mediator().record_evaluation_info() {
            self.mediator().run_result().write_quorum_accesses += 1;
        }
        let mut register_array_being_written =
            self.register_array_being_written.lock().unwrap();

        while register_array_being_written.is_some() {
            let timeout = Duration::from_millis(100); // TODO: Have as a parameter somewhere
            let result = self
                .write_ack_majority_reached
                .wait_timeout(register_array_being_written, timeout)
                .expect("Error when waiting on write ack Condvar");
            register_array_being_written = result.0;
            if result.1.timed_out() {
                self.broadcast_json_message(&json_write_message);
                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().write_quorum_accesses +=
                        1;
                }
            }
        }
    }

    fn send_json_message_to(&self, json: &str, receiver_id: NodeId) {
        self.mediator().send_json_to(json, receiver_id);

        if self.mediator().record_evaluation_info() {
            if messages::json_is_write_message(json) {
                self.mediator().run_result().write_message.sent += 1;
            } else if messages::json_is_write_ack_message(json) {
                self.mediator().run_result().write_ack_message.sent += 1;
            } else if messages::json_is_read_message(json) {
                self.mediator().run_result().read_message.sent += 1;
            } else if messages::json_is_read_ack_message(json) {
                self.mediator().run_result().read_ack_message.sent += 1;
            }
        }
    }

    fn broadcast_json_message(&self, json: &str) {
        self.mediator().broadcast_json(json);

        if self.mediator().record_evaluation_info() {
            if messages::json_is_write_message(json) {
                self.mediator().run_result().write_message.sent +=
                    self.mediator().number_of_nodes();
            } else if messages::json_is_write_ack_message(json) {
                self.mediator().run_result().write_ack_message.sent +=
                    self.mediator().number_of_nodes();
            } else if messages::json_is_read_message(json) {
                self.mediator().run_result().read_message.sent +=
                    self.mediator().number_of_nodes();
            } else if messages::json_is_read_ack_message(json) {
                self.mediator().run_result().read_ack_message.sent +=
                    self.mediator().number_of_nodes();
            }
        }
    }

    fn mediator(&self) -> Arc<M> {
        self.mediator
            .upgrade()
            .expect("Error upgrading mediator in AbdNode")
    }

    #[allow(dead_code)]
    pub fn read(&self, node_id: NodeId) -> V {
        let result = self.inner_read();
        result.get(node_id).clone().val
    }

    #[allow(dead_code)]
    pub fn read_all(&self) -> RegisterArray<V> {
        let result = self.inner_read();
        result.clone()
    }

    fn inner_read(&self) -> MutexGuard<RegisterArray<V>> {
        let register_array = self.acquire_register_array_and_clone_it_to_register_array_being_read();
        let json_read_message = self
            .construct_json_read_message_and_release_register_array(
                register_array,
            );

        self.broadcast_json_read_message_until_majority_has_acked(
            &json_read_message,
        );

        if cfg!(debug_assertions) {
            let acking_processors_for_read =
                self.acking_processors_for_read.lock().unwrap();
            let register_array_being_read =
                self.register_array_being_read.lock().unwrap();
            assert!(acking_processors_for_read.is_empty());
            assert!(register_array_being_read.is_none());
        }

        if self.mediator().record_evaluation_info() {
            self.mediator().run_result().read_ops += 1;
        }

        self.reg.lock().unwrap()
    }

    fn acquire_register_array_and_clone_it_to_register_array_being_read(
        &self,
    ) -> MutexGuard<RegisterArray<V>> {
        let register_array = self.reg.lock().unwrap();
        let mut register_array_being_read =
            self.register_array_being_read.lock().unwrap();
        *register_array_being_read = Some(register_array.clone());
        register_array
    }

    fn construct_json_read_message_and_release_register_array(
        &self,
        register_array: MutexGuard<RegisterArray<V>>,
    ) -> String {
        let read_message = ReadMessage {
            sender: self.mediator().node_id(),
            register_array: Cow::Borrowed(&register_array),
        };

        self.jsonify_message(&read_message)
    }

    fn broadcast_json_read_message_until_majority_has_acked(
        &self,
        json_read_message: &str,
    ) {
        self.broadcast_json_message(&json_read_message);
        if self.mediator().record_evaluation_info() {
            self.mediator().run_result().read_quorum_accesses += 1;
        }

        let mut register_array_being_read =
            self.register_array_being_read.lock().unwrap();

        while register_array_being_read.is_some() {
            let timeout = Duration::from_millis(100); // TODO: Have as a parameter somewhere
            let result = self
                .read_ack_majority_reached
                .wait_timeout(register_array_being_read, timeout)
                .expect("Error waiting on read ack condvar");
            register_array_being_read = result.0;
            if result.1.timed_out() {
                self.broadcast_json_message(&json_read_message);
                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().read_quorum_accesses += 1;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn json_received(&self, json: &str) {
        if self.mediator().record_evaluation_info() {
            if messages::json_is_write_message(json) {
                self.mediator().run_result().write_message.received += 1;
            } else if messages::json_is_write_ack_message(json) {
                self.mediator().run_result().write_ack_message.received +=
                    1;
            } else if messages::json_is_read_message(json) {
                self.mediator().run_result().read_message.received += 1;
            } else if messages::json_is_read_ack_message(json) {
                self.mediator().run_result().read_ack_message.received +=
                    1;
            }
        }

        if messages::json_is_write_message(json) {
            if let Ok(write_message) = serde_json::from_str(&json) {
                return self.receive_write_message(write_message);
            }
        }

        if messages::json_is_write_ack_message(json) {
            if let Ok(write_ack_message) = serde_json::from_str(&json) {
                return self.receive_write_ack_message(write_ack_message);
            }
        }

        if messages::json_is_read_message(json) {
            if let Ok(read_message) = serde_json::from_str(&json) {
                return self.receive_read_message(read_message);
            }
        }

        if messages::json_is_read_ack_message(json) {
            if let Ok(read_ack_message) = serde_json::from_str(&json) {
                return self.receive_read_ack_message(read_ack_message);
            }
        }

        printlnu(format!("Could not parse the json: {}", json));
    }

    fn receive_write_message(&self, write_message: WriteMessage<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register_array(
            &write_message.register_array,
        );

        let write_ack_message = WriteAckMessage {
            sender: self.mediator().node_id(),
            register_array: Cow::Borrowed(&reg),
        };

        let json = self.jsonify_message(&write_ack_message);
        self.send_json_message_to(&json, write_message.sender);

        // Here we have a compromise. Either we lock reg for
        // a long time, or we clone reg so we can have more
        // concurrency. For small entries, cloning might be
        // better. For large entries, longer locking
        // might be better.

        if self.mediator().record_evaluation_info() {
            self.mediator()
                .run_result()
                .write_message
                .nodes_received_from
                .insert(write_message.sender);
        }
    }

    fn receive_write_ack_message(
        &self,
        write_ack_message: WriteAckMessage<V>,
    ) {
        let received_register_array: &RegisterArray<V> =
            &write_ack_message.register_array;
        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register_array(received_register_array);
        }

        let mut register_array_being_written =
            self.register_array_being_written.lock().unwrap();
        let mut received_register_array_was_at_least_as_large = false;

        if let Some(register_array_being_written) =
            &*register_array_being_written
        {
            if received_register_array >= register_array_being_written {
                let mut acking_processors_for_write =
                    self.acking_processors_for_write.lock().unwrap();
                acking_processors_for_write
                    .insert(write_ack_message.sender);

                received_register_array_was_at_least_as_large = true;
            }
        }

        if received_register_array_was_at_least_as_large
            && self.write_ack_from_majority()
        {
            let mut acking_processors_for_write =
                self.acking_processors_for_write.lock().unwrap();
            acking_processors_for_write.clear();
            *register_array_being_written = None;

            self.write_ack_majority_reached.notify_one();
        }

        if self.mediator().record_evaluation_info() {
            self.mediator()
                .run_result()
                .write_ack_message
                .nodes_received_from
                .insert(write_ack_message.sender);
        }
    }

    fn write_ack_from_majority(&self) -> bool {
        let acking_processors_for_write =
            self.acking_processors_for_write.lock().unwrap();

        acking_processors_for_write.len() as Int
            >= self.number_of_nodes_in_a_majority()
    }

    fn number_of_nodes_in_a_majority(&self) -> Int {
        self.mediator().number_of_nodes() / 2 + 1
    }

    fn receive_read_message(&self, read_message: ReadMessage<V>) {
        let mut reg = self.reg.lock().unwrap();
        reg.merge_to_max_from_register_array(&read_message.register_array);

        let read_ack_message = ReadAckMessage {
            sender: self.mediator().node_id(),
            register_array: Cow::Borrowed(&reg),
        };

        let json = self.jsonify_message(&read_ack_message);
        self.send_json_message_to(&json, read_message.sender);

        if self.mediator().record_evaluation_info() {
            self.mediator()
                .run_result()
                .read_message
                .nodes_received_from
                .insert(read_message.sender);
        }
    }

    fn receive_read_ack_message(
        &self,
        read_ack_message: ReadAckMessage<V>,
    ) {
        let received_register_array: &RegisterArray<V> =
            &read_ack_message.register_array;
        {
            let mut reg = self.reg.lock().unwrap();
            reg.merge_to_max_from_register_array(&received_register_array);
        }

        let mut register_array_being_read =
            self.register_array_being_read.lock().unwrap();
        let mut received_register_array_was_at_least_as_large = false;

        if let Some(register_array_being_read) =
            &*register_array_being_read
        {
            if received_register_array >= register_array_being_read {
                let mut acking_processors_for_read =
                    self.acking_processors_for_read.lock().unwrap();
                acking_processors_for_read.insert(read_ack_message.sender);

                received_register_array_was_at_least_as_large = true;
            }
        }

        if received_register_array_was_at_least_as_large
            && self.read_ack_from_majority()
        {
            let mut acking_processors_for_read =
                self.acking_processors_for_read.lock().unwrap();
            acking_processors_for_read.clear();

            *register_array_being_read = None;
            self.read_ack_majority_reached.notify_one();
        }

        if self.mediator().record_evaluation_info() {
            self.mediator()
                .run_result()
                .read_ack_message
                .nodes_received_from
                .insert(read_ack_message.sender);
        }
    }

    fn read_ack_from_majority(&self) -> bool {
        let acking_processors_for_read =
            self.acking_processors_for_read.lock().unwrap();

        acking_processors_for_read.len() as Int
            >= self.number_of_nodes_in_a_majority()
    }
    */
}
