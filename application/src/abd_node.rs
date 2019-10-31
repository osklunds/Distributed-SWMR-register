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
use crate::messages::{
    self, Message, TimestampValueMessage, WriteAckMessage, WriteMessage,
    Read1Message, Read1AckMessage, Read2Message, Read2AckMessage
};
use crate::quorum::Quorum;
use crate::terminal_output::printlnu;

#[cfg(test)]
pub mod tests;

pub struct AbdNode<M, V> {
    mediator: Weak<M>,

    timestamp: Mutex<Timestamp>,
    value: Mutex<V>,
    read1_sequence_number: Mutex<Timestamp>,
    read2_sequence_number: Mutex<Timestamp>,

    write_quorum: Quorum,
    read1_quorum: Quorum,
    read2_quorum: Quorum,
}

pub trait Value:
    Default + Serialize + DeserializeOwned + Debug + Clone
{
}
impl<V: Default + Serialize + DeserializeOwned + Debug + Clone> Value
    for V
{
}

impl<V: Value, M: Med> AbdNode<M, V> {
    //
    // Basic functions
    //

    pub fn new(mediator: Weak<M>) -> AbdNode<M, V> {
        let mediator_upgraded = mediator
            .upgrade()
            .expect("Error upgrading mediator in AbdNode constructor");
        let node_ids = mediator_upgraded.node_ids();
        let number_of_nodes = mediator_upgraded.number_of_nodes();

        AbdNode {
            mediator: mediator,

            timestamp: Mutex::new(0),
            value: Mutex::new(V::default()),

            read1_sequence_number: Mutex::new(1),
            read2_sequence_number: Mutex::new(1),

            write_quorum: Quorum::new(number_of_nodes),
            read1_quorum: Quorum::new(number_of_nodes),
            read2_quorum: Quorum::new(number_of_nodes),
        }
    }

    fn mediator(&self) -> Arc<M> {
        self.mediator
            .upgrade()
            .expect("Error upgrading mediator in AbdNode")
    }

    //
    // Write client-side
    //

    pub fn write(&self, value: V) {
        if cfg!(debug_assertions) {
            assert!(self.write_quorum.is_idle());
        }

        self.write_inner(value);

        if cfg!(debug_assertions) {
            assert!(self.write_quorum.is_idle());
        }

        if self.mediator().record_evaluation_info() {
            self.mediator().run_result().write_ops += 1;
        }
    }

    fn write_inner(&self, new_value: V) {
        self.update_local_timestamp_and_value(new_value);
        let write_message = self.construct_write_message();
        self.quorum_access(&write_message, &self.write_quorum);
    }

    fn update_local_timestamp_and_value(&self, new_value: V) {
        let mut timestamp = self.timestamp.lock().unwrap();
        *timestamp += 1;

        let mut value = self.value.lock().unwrap();
        *value = new_value;
    }

    fn construct_write_message(&self) -> WriteMessage<V> {
        let mut timestamp = self.timestamp.lock().unwrap();
        let mut value = self.value.lock().unwrap();

        WriteMessage {
            sender: self.mediator().node_id(),
            timestamp: *timestamp,
            value: value.clone(),
        }
    }

    fn quorum_access<Msg: Message>(&self, message: &Msg, quorum: &Quorum) {
        let json = self.jsonify_message(message);
        self.broadcast_json(&json);

        let mut accessing = quorum.accessing().lock().unwrap();
        *accessing = true;
        while *accessing {
            let result = quorum
                .majority_reached()
                .wait_timeout(accessing, Duration::from_millis(100))
                .unwrap();
            accessing = result.0;
            if result.1.timed_out() {
                self.broadcast_json(&json);
            }
        }
    }

    //
    // Write server-side
    //

    fn receive_write_message(&self, write_message: &WriteMessage<V>) {
        self.update_local_timestamp_and_value_from_message(write_message);
        let write_ack_message =
            self.write_ack_message_from_write_message(write_message);
        self.send_message_to(&write_ack_message, write_message.sender);
    }

    fn update_local_timestamp_and_value_from_message<
        TVM: TimestampValueMessage<V>,
    >(
        &self,
        message: &TVM,
    ) {
        let mut timestamp = self.timestamp.lock().unwrap();
        let mut value = self.value.lock().unwrap();

        if message.timestamp() > *timestamp {
            *timestamp = message.timestamp();
            *value = message.value().clone();
        }
    }

    fn write_ack_message_from_write_message(
        &self,
        write_message: &WriteMessage<V>,
    ) -> WriteAckMessage {
        WriteAckMessage {
            sender: self.mediator().node_id(),
            timestamp: write_message.timestamp,
        }
    }

    fn receive_write_ack_message(
        &self,
        write_ack_message: &WriteAckMessage,
    ) {
        let timestamp = self.timestamp.lock().unwrap();

        if write_ack_message.timestamp == *timestamp {
            let mut acking_processors =
                self.write_quorum.acking_processors().lock().unwrap();
            acking_processors.insert(write_ack_message.sender);
        }

        self.write_quorum.notify_if_has_ack_from_majority();
    }

    
    //
    // Read client-side
    //

    pub fn read(&self) -> V {
        self.read_phase1();
        self.read_phase2();

        self.value.lock().unwrap().clone()
    }

    fn read_phase1(&self) {
        let mut sequence_number = self.read1_sequence_number.lock().unwrap();
        *sequence_number += 1;

        let read1_message = Read1Message {
            sender: self.mediator().node_id(), 
            sequence_number: *sequence_number,
        };
        self.quorum_access(&read1_message, &self.read1_quorum);
    }

    fn read_phase2(&self) {
        let mut sequence_number = self.read2_sequence_number.lock().unwrap();
        *sequence_number += 1;
        let timestamp = self.timestamp.lock().unwrap();
        let value = self.value.lock().unwrap();

        let read2_message = Read2Message {
            sender: self.mediator().node_id(), 
            timestamp: *timestamp, 
            value: value.clone(), 
            sequence_number: *sequence_number,
        };
        self.quorum_access(&read2_message, &self.read2_quorum);
    }

    //
    // Read server-side
    //

    fn receive_read1_message(&self, read1_message: &Read1Message) {
        let timestamp = self.timestamp.lock().unwrap();
        let value = self.value.lock().unwrap();

        let read1_ack_message = Read1AckMessage {
            sender: self.mediator().node_id(),
            timestamp: *timestamp,
            value: value.clone(),
            sequence_number: read1_message.sequence_number
        };

        self.send_message_to(&read1_ack_message, read1_message.sender);
    }

    fn receive_read1_ack_message(&self, read1_ack_message: &Read1AckMessage<V>) {
        let sequence_number = self.read1_sequence_number.lock().unwrap();

        if read1_ack_message.sequence_number == *sequence_number {
            self.update_local_timestamp_and_value_from_message(read1_ack_message);

            let mut acking_processors = self.read1_quorum.acking_processors().lock().unwrap();
            acking_processors.insert(read1_ack_message.sender);
        }

        self.read1_quorum.notify_if_has_ack_from_majority();
    }

    fn receive_read2_message(&self, read2_message: &Read2Message<V>) {
        self.update_local_timestamp_and_value_from_message(read2_message);
        let read2_ack_message = Read2AckMessage {
            sender: self.mediator().node_id(),
            sequence_number: read2_message.sequence_number
        };
        self.send_message_to(&read2_ack_message, read2_message.sender);
    }

    fn receive_read2_ack_message(&self, read2_ack_message: &Read2AckMessage) {
        let sequence_number = self.read2_sequence_number.lock().unwrap();

        if read2_ack_message.sequence_number == *sequence_number {
            let mut acking_processors =
                self.read2_quorum.acking_processors().lock().unwrap();
            acking_processors.insert(read2_ack_message.sender);
        }

        self.read2_quorum.notify_if_has_ack_from_majority();
    }




    //
    // Message sending
    //

    fn jsonify_message<Msg: Message>(&self, message: &Msg) -> String {
        serde_json::to_string(message)
            .expect("Could not serialize a message")
    }

    fn broadcast_json(&self, json: &str) {
        self.mediator().broadcast_json(json);

        if self.mediator().record_evaluation_info() {
            if messages::json_is_write_message(json) {
                self.mediator().run_result().write_message.sent +=
                    self.mediator().number_of_nodes();
            } else if messages::json_is_write_ack_message(json) {
                self.mediator().run_result().write_ack_message.sent +=
                    self.mediator().number_of_nodes();
            } /* else if messages::json_is_read_message(json) {
                  self.mediator().run_result().read_message.sent += 1;
              } else if messages::json_is_read_ack_message(json) {
                  self.mediator().run_result().read_ack_message.sent += 1;
              } */
        }
    }

    fn send_message_to<Msg: Message>(
        &self,
        message: &Msg,
        receiver_id: NodeId,
    ) {
        let json = self.jsonify_message(message);
        self.mediator().send_json_to(&json, receiver_id);

        if self.mediator().record_evaluation_info() {
            if messages::json_is_write_message(&json) {
                self.mediator().run_result().write_message.sent += 1;
            } else if messages::json_is_write_ack_message(&json) {
                self.mediator().run_result().write_ack_message.sent += 1;
            } /* else if messages::json_is_read_message(json) {
                  self.mediator().run_result().read_message.sent += 1;
              } else if messages::json_is_read_ack_message(json) {
                  self.mediator().run_result().read_ack_message.sent += 1;
              } */
        }
    }


    //
    // json message handling
    //

    pub fn json_received(&self, json: &str) {
        self.try_receive_write_message_json(json);
        self.try_receive_write_ack_message_json(json);
        self.try_receive_read1_message_json(json);
        self.try_receive_read1_ack_message_json(json);
    }

    fn try_receive_write_message_json(&self, json: &str) {
        if messages::json_is_write_message(json) {
            if let Ok(write_message) = serde_json::from_str(&json) {
                self.receive_write_message(&write_message);

                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().write_message.received +=
                        1;
                    self.mediator()
                        .run_result()
                        .write_message
                        .nodes_received_from
                        .insert(write_message.sender);
                }
            }
        }
    }

    fn try_receive_write_ack_message_json(&self, json: &str) {
        if messages::json_is_write_ack_message(json) {
            if let Ok(write_ack_message) = serde_json::from_str(&json) {
                self.receive_write_ack_message(&write_ack_message);

                if self.mediator().record_evaluation_info() {
                    self.mediator()
                        .run_result()
                        .write_ack_message
                        .received += 1;
                    self.mediator()
                        .run_result()
                        .write_ack_message
                        .nodes_received_from
                        .insert(write_ack_message.sender);
                }
            }
        }
    }

    fn try_receive_read1_message_json(&self, json: &str) {
        if messages::json_is_read1_message(json) {
            if let Ok(read1_message) = serde_json::from_str(&json) {
                self.receive_read1_message(&read1_message);

                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().read_message.received +=
                        1;
                    self.mediator()
                        .run_result()
                        .read_message
                        .nodes_received_from
                        .insert(read1_message.sender);
                }
            }
        }
    }

    fn try_receive_read1_ack_message_json(&self, json: &str) {
        if messages::json_is_read1_ack_message(json) {
            if let Ok(read1_ack_message) = serde_json::from_str(&json) {
                self.receive_read1_ack_message(&read1_ack_message);

                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().read_ack_message.received +=
                        1;
                    self.mediator()
                        .run_result()
                        .read_ack_message
                        .nodes_received_from
                        .insert(read1_ack_message.sender);
                }
            }
        }
    }

    fn try_receive_read2_message_json(&self, json: &str) {
        if messages::json_is_read2_message(json) {
            if let Ok(read2_message) = serde_json::from_str(&json) {
                self.receive_read2_message(&read2_message);

                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().read_message.received +=
                        1;
                    self.mediator()
                        .run_result()
                        .read_message
                        .nodes_received_from
                        .insert(read2_message.sender);
                }
            }
        }
    }

    fn try_receive_read2_ack_message_json(&self, json: &str) {
        if messages::json_is_read2_ack_message(json) {
            if let Ok(read2_ack_message) = serde_json::from_str(&json) {
                self.receive_read2_ack_message(&read2_ack_message);

                if self.mediator().record_evaluation_info() {
                    self.mediator().run_result().read_ack_message.received +=
                        1;
                    self.mediator()
                        .run_result()
                        .read_ack_message
                        .nodes_received_from
                        .insert(read2_ack_message.sender);
                }
            }
        }
    }
}