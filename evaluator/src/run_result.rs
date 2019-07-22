
use std::collections::HashSet;

use serde::{Serialize, Deserialize};


pub type ScenarioResult = Vec<RunResult>;
pub type ScenarioResults = HashSet<ScenarioResult>;


pub type NodeId = i32;


#[derive(Serialize, Deserialize, Debug)]
pub struct RunResult {
    pub write_ops: usize,
    pub write_quorum_accesses: usize,
    pub read_ops: usize,
    pub read_quorum_accesses: usize,

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
            read_ops: 0,
            read_quorum_accesses: 0,

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
    pub sent: usize,
    pub received: usize,
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
    pub is_writer: bool,
    pub run_length: usize,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            node_id: 0,
            is_reader: false,
            is_writer: false,
            run_length: 0
        }
    }
}