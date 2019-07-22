
use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::settings::NodeId;


#[derive(Serialize, Deserialize, Debug)]
pub struct RunResult {
    pub write_ops: i32,
    pub write_quorum_accesses: i32,
    pub write_latency: f32,
    pub read_ops: i32,
    pub read_quorum_accesses: i32,
    pub read_latency: f32,

    pub write_message: MessageTypeResult,
    pub write_ack_message: MessageTypeResult,
    pub read_message: MessageTypeResult,
    pub read_ack_message: MessageTypeResult
}

impl RunResult {
    pub fn new() -> RunResult {
        RunResult {
            write_ops: 0,
            write_quorum_accesses: 0,
            write_latency: 0.0,
            read_ops: 0,
            read_quorum_accesses: 0,
            read_latency: 0.0,

            write_message: MessageTypeResult::new(),
            write_ack_message: MessageTypeResult::new(),
            read_message: MessageTypeResult::new(),
            read_ack_message: MessageTypeResult::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageTypeResult {
    pub sent: i32,
    pub received: i32,
    pub nodes_received_from: HashSet<NodeId>
}

impl MessageTypeResult {
    pub fn new() -> MessageTypeResult {
        MessageTypeResult {
            sent: 0,
            received: 0,
            nodes_received_from: HashSet::new()
        }
    }
}