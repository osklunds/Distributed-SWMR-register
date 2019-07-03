
use serde::{Serialize, Deserialize};

use crate::register::Register;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct WriteMessage<V> {
    #[serde(rename = "WriteMessage")]
    pub sender: i32,
    pub register: Register<V>
}

// Idea: custom serialization. Try to deserialize to the different classes.

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct WriteAckMessage<V> {
    #[serde(rename = "WriteAckMessage")]
    pub sender: i32,
    pub register: Register<V>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct ReadMessage<V> {
    #[serde(rename = "ReadMessage")]
    pub sender: i32,
    pub register: Register<V>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct ReadAckMessage<V> {
    #[serde(rename = "ReadAckMessage")]
    pub sender: i32,
    pub register: Register<V>
}
