
use serde::{Serialize, Deserialize};
use std::borrow::Cow;


use crate::register::Register;


pub trait Message : Serialize {}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WriteMessage<'a, V: Clone> {
    #[serde(rename = "WriteMessage")]
    pub sender: i32,
    pub register: Cow<'a, Register<V>>
}

impl<'a, V: Serialize + Clone> Message for WriteMessage<'a, V> {}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WriteAckMessage<'a, V: Clone> {
    #[serde(rename = "WriteAckMessage")]
    pub sender: i32,
    pub register: Cow<'a, Register<V>>
}

impl<'a, V: Serialize + Clone> Message for WriteAckMessage<'a, V> {}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ReadMessage<'a, V: Clone> {
    #[serde(rename = "ReadMessage")]
    pub sender: i32,
    pub register: Cow<'a, Register<V>>
}

impl<'a, V: Serialize + Clone> Message for ReadMessage<'a, V> {}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ReadAckMessage<'a, V: Clone> {
    #[serde(rename = "ReadAckMessage")]
    pub sender: i32,
    pub register: Cow<'a, Register<V>>
}

impl<'a, V: Serialize + Clone> Message for ReadAckMessage<'a, V> {}


pub fn json_is_write_message(json: &str) -> bool {
    json.starts_with("{\"WriteMessage\":")
}

pub fn json_is_write_ack_message(json: &str) -> bool {
    json.starts_with("{\"WriteAckMessage\":")
}

pub fn json_is_read_message(json: &str) -> bool {
    json.starts_with("{\"ReadMessage\":")
}

pub fn json_is_read_ack_message(json: &str) -> bool {
    json.starts_with("{\"ReadAckMessage\":")
}