use std::collections::HashSet;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

use crate::types::{Int, NodeId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunResult {
    pub write_ops: Int,
    pub write_quorum_accesses: Int,
    pub read_ops: Int,
    pub read_quorum_accesses: Int,

    pub write_message: MessageTypeResult,
    pub write_ack_message: MessageTypeResult,
    pub read1_message: MessageTypeResult,
    pub read1_ack_message: MessageTypeResult,
    pub read2_message: MessageTypeResult,
    pub read2_ack_message: MessageTypeResult,

    pub metadata: Metadata,
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
            read1_message: MessageTypeResult::new(),
            read1_ack_message: MessageTypeResult::new(),
            read2_message: MessageTypeResult::new(),
            read2_ack_message: MessageTypeResult::new(),

            metadata: Metadata::new(),
        }
    }

    pub fn is_sound(&self, number_of_nodes: Int) -> bool {
        let mut sound = true;

        sound &= Self::implies(
            self.metadata.is_writer,
            self.write_ack_message.nodes_received_from
                == Self::all_nodes_set(number_of_nodes),
        );

        sound &= Self::implies(
            self.metadata.is_reader,
            self.read1_ack_message.nodes_received_from
                == Self::all_nodes_set(number_of_nodes),
        );

        sound &= Self::implies(
            self.metadata.is_reader,
            self.read2_ack_message.nodes_received_from
                == Self::all_nodes_set(number_of_nodes),
        );

        sound &= Self::implies(self.metadata.is_writer, !self.metadata.is_reader);

        sound &= Self::implies(self.metadata.is_reader, !self.metadata.is_writer);

        sound &= Self::implies(
            !self.metadata.is_writer,
            self.write_ack_message.nodes_received_from.is_empty(),
        );

        sound &= Self::implies(
            !self.metadata.is_reader,
            self.read1_ack_message.nodes_received_from.is_empty(),
        );

        sound &= Self::implies(
            !self.metadata.is_reader,
            self.read2_ack_message.nodes_received_from.is_empty(),
        );

        sound
    }

    fn implies(a: bool, b: bool) -> bool {
        if a {
            b
        } else {
            true
        }
    }

    fn all_nodes_set(number_of_nodes: Int) -> HashSet<NodeId> {
        HashSet::from_iter(1..(number_of_nodes + 1) as NodeId)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageTypeResult {
    pub sent: Int,
    pub received: Int,
    pub nodes_received_from: HashSet<NodeId>,
}

impl MessageTypeResult {
    pub fn new() -> MessageTypeResult {
        MessageTypeResult {
            sent: 0,
            received: 0,
            nodes_received_from: HashSet::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub node_id: NodeId,
    pub is_reader: bool,
    pub is_writer: bool,
    pub run_length: Int,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            node_id: 0,
            is_reader: false,
            is_writer: false,
            run_length: 0,
        }
    }
}
