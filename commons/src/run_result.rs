
use std::collections::HashSet;
use std::iter::FromIterator;

use serde::{Serialize, Deserialize};

use crate::types::NodeId;


#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[allow(dead_code)]
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

    pub fn is_valid(&self, number_of_nodes: usize) -> bool {
        let mut valid = true;
        
        valid &= Self::implies(self.metadata.is_writer, self.write_ack_message.nodes_received_from == Self::all_nodes_set(number_of_nodes));

        valid &= Self::implies(self.metadata.is_reader, self.read_ack_message.nodes_received_from == Self::all_nodes_set(number_of_nodes));

        valid &= Self::implies(!self.metadata.is_writer, self.write_ack_message.nodes_received_from.is_empty());

        valid &= Self::implies(!self.metadata.is_reader, self.read_ack_message.nodes_received_from.is_empty());

        valid
    }

    fn implies(a: bool, b: bool) -> bool {
        if a {
            b
        } else {
            true
        }
    }

    fn all_nodes_set(number_of_nodes: usize) -> HashSet<NodeId> {
        HashSet::from_iter(1..(number_of_nodes+1) as NodeId)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageTypeResult {
    pub sent: usize,
    pub received: usize,
    pub nodes_received_from: HashSet<NodeId>
}

impl MessageTypeResult {
    #[allow(dead_code)]
    pub fn new() -> MessageTypeResult {
        MessageTypeResult {
            sent: 0,
            received: 0,
            nodes_received_from: HashSet::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub node_id: NodeId,
    pub is_reader: bool,
    pub is_writer: bool,
    pub run_length: usize,
}

impl Metadata {
    #[allow(dead_code)]
    pub fn new() -> Metadata {
        Metadata {
            node_id: 0,
            is_reader: false,
            is_writer: false,
            run_length: 0
        }
    }
}