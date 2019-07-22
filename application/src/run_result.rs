
use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::settings::NodeId;


#[derive(Serialize, Deserialize, Debug)]
pub struct RunResult {
    pub write_ops: i32,
    pub write_quorum_accesses: i32,
    pub write_latency: Option<f32>,
    pub read_ops: i32,
    pub read_quorum_accesses: i32,
    pub read_latency: Option<f32>,

    // The latencies are Option, because if a node doesn't write,
    // it makes no sense to say that it has a write latency.
    // But even if a node doesn't write, it still makes sense to say
    // that it hasn't done any write quorum accesses.

    pub write_message: MessageTypeResult,
    pub write_ack_message: MessageTypeResult,
    pub read_message: MessageTypeResult,
    pub read_ack_message: MessageTypeResult,

    pub metadata: Metadata
}

impl RunResult {
    pub fn new() -> RunResult {
        RunResult {
            write_ops: 0,
            write_quorum_accesses: 0,
            write_latency: None,
            read_ops: 0,
            read_quorum_accesses: 0,
            read_latency: None,

            write_message: MessageTypeResult::new(),
            write_ack_message: MessageTypeResult::new(),
            read_message: MessageTypeResult::new(),
            read_ack_message: MessageTypeResult::new(),

            metadata: Metadata::new()
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub node_id: NodeId,
    pub is_reader: bool,
    pub is_writer: bool
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            node_id: 0,
            is_reader: false,
            is_writer: false
        }
    }
}