
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RunResult {
    write_ops: i32,
    write_quorum_accesses: i32,
    write_latency: f32,
    read_ops: i32,
    read_quorum_accesses: i32,
    read_latency: f32,

    write_message: MessageTypeResult,
    write_ack_message: MessageTypeResult,
    read_message: MessageTypeResult,
    read_ack_message: MessageTypeResult
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
    sent: i32,
    received: i32
}

impl MessageTypeResult {
    pub fn new() -> MessageTypeResult {
        MessageTypeResult {
            sent: 0,
            received: 0
        }
    }
}