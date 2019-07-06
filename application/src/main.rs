
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

mod register;
mod node;
mod messages;
mod settings;
mod terminal_output;

use std::time::SystemTime;

use std::str;

use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::sync::mpsc::channel;

use std::collections::{HashMap, HashSet};

use std::default::Default;

use std::env;

use serde_json;
use serde::{Serialize, Deserialize};

use colored::*;

use clap::{Arg, App, SubCommand};


use node::Node;
use messages::WriteMessage;
use messages::ReadMessage;
use register::Register;
use register::Entry;
use settings::SETTINGS;
use terminal_output::printlnu;


fn main() {
    let node_id = SETTINGS.node_id;
    let socket_addrs = SETTINGS.socket_addrs.clone();

    let node: Node<String> = Node::new(node_id, socket_addrs).unwrap();
    let node = Arc::new(node);

    let (tx,rx) = channel();

    let recv_thread_node = Arc::clone(&node);
    let recv_thread_handle = thread::spawn(move || {
        recv_thread_node.set_send_end(tx);
        recv_thread_node.recv_loop();
    });

    let client_op_thread_node = Arc::clone(&node);
    let client_op_thread_handle = thread::spawn(move || {
        client_op_thread_node.set_receive_end(rx);

        if node_id == 1 {
            // Temp hack to wait for the other nodes to start
            thread::sleep(time::Duration::from_millis(2000));

            let start = SystemTime::now();

            for _ in 0..1000 {
                client_op_thread_node.write(format!("Hej"));
                //printlnu(format!("{}", i));
            }

            let elapsed = start.elapsed().unwrap();

            printlnu(format!("{}", elapsed.as_millis()));

        }
    });

    recv_thread_handle.join().unwrap();
    client_op_thread_handle.join().unwrap();
}
