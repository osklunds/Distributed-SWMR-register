
use serde::{Serialize, Deserialize};

use crate::register::Register;


#[derive(Serialize, Deserialize, Debug)]
pub struct Message<V> {
    pub message_type: MessageType,
    pub register: Register<V>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    WriteMessage,
    WriteAckMessage,
    ReadMessage,
    ReadAckMessage
}
