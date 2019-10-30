use serde::{Deserialize, Serialize};

use commons::types::NodeId;

use super::data_types::timestamp::Timestamp;

pub trait Message: Serialize {}

pub trait TimestampValueMessage<V> {
    fn timestamp(&self) -> Timestamp;
    fn value(&self) -> &V;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WriteMessage<V> {
    #[serde(rename = "WriteMessage")]
    pub sender: NodeId,
    pub timestamp: Timestamp,
    pub value: V,
}

impl<V> WriteMessage<V> {
    pub fn new(
        sender: NodeId,
        timestamp: Timestamp,
        value: V,
    ) -> WriteMessage<V> {
        WriteMessage {
            sender: sender,
            timestamp: timestamp,
            value: value,
        }
    }
}

impl<V: Serialize> Message for WriteMessage<V> {}

impl<V> TimestampValueMessage<V> for WriteMessage<V> {
    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn value(&self) -> &V {
        &self.value
    }
}

pub fn json_is_write_message(json: &str) -> bool {
    json.starts_with("{\"WriteMessage\":")
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WriteAckMessage {
    #[serde(rename = "WriteAckMessage")]
    pub sender: NodeId,
    pub timestamp: Timestamp,
}

impl WriteAckMessage {
    pub fn new(sender: NodeId, timestamp: Timestamp) -> WriteAckMessage {
        WriteAckMessage {
            sender: sender,
            timestamp: timestamp,
        }
    }
}

impl Message for WriteAckMessage {}

pub fn json_is_write_ack_message(json: &str) -> bool {
    json.starts_with("{\"WriteAckMessage\":")
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Read1Message {
    #[serde(rename = "Read1Message")]
    pub sender: NodeId,
    pub timestamp: Timestamp,
}

impl Message for Read1Message {}

pub fn json_is_read1_message(json: &str) -> bool {
    json.starts_with("{\"Read1Message\":")
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Read1AckMessage<V> {
    #[serde(rename = "Read1AckMessage")]
    pub sender: NodeId,
    pub timestamp: Timestamp,
    pub value: V,
}

impl<V: Serialize> Message for Read1AckMessage<V> {}

pub fn json_is_read1_ack_message(json: &str) -> bool {
    json.starts_with("{\"Read1AckMessage\":")
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Read2Message<V> {
    #[serde(rename = "Read2Message")]
    pub sender: NodeId,
    pub timestamp: Timestamp,
    pub value: V,
}

impl<V: Serialize> Message for Read2Message<V> {}

pub fn json_is_read2_message(json: &str) -> bool {
    json.starts_with("{\"Read2Message\":")
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Read2AckMessage {
    #[serde(rename = "Read2AckMessage")]
    pub sender: NodeId,
    pub timestamp: Timestamp,
}

impl Message for Read2AckMessage {}

pub fn json_is_read2_ack_message(json: &str) -> bool {
    json.starts_with("{\"Read2AckMessage\":")
}

// TODO: Tests
