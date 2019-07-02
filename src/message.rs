
use serde::{Serialize, Deserialize};

use crate::register::Register;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Message<V> {
    pub sender: i32,
    pub message_type: MessageType,
    pub register: Register<V>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum MessageType {
    WriteMessage,
    WriteAckMessage,
    ReadMessage,
    ReadAckMessage
}
