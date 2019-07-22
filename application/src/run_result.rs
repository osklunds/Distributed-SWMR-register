
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

pub struct MessageTypeResult {
    sent: i32,
    received: i32
}