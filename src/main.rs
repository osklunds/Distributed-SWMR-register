
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

extern crate serde;

mod register;
mod node;
mod messages;

use std::str;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use std::collections::{HashMap, HashSet};

use std::default::Default;

use std::env;

use serde_json;
use serde::{Serialize, Deserialize};

use node::Node;
use messages::WriteMessage;
use messages::ReadMessage;
use register::Register;
use register::Entry;

    fn node_ids_for_tests() -> HashSet<i32> {
        let mut node_ids = HashSet::new();
        node_ids.insert(1);
        node_ids.insert(2);
        node_ids.insert(3);
        node_ids.insert(4);
        node_ids
    }

fn register_for_tests() -> Register<String> {
        Register::new(node_ids_for_tests())
    }

fn main() {
    let set = node_ids_for_tests();
    //let x: Vec<i32> = set.iter().collect();

let five_fives = std::iter::repeat(5).take(5);

let v: Vec<i32> = five_fives.collect();

    /*
    let mut nodes = HashSet::new();
    nodes.insert(3);
    let reg: Register<String> = Register::new(nodes);
    let m = WriteMessage {
        sender: 8,
        register: reg
    };

    let json_string = serde_json::to_string(&m).unwrap();

    println!("{}", json_string);

    let x: Result<ReadMessage<String>, serde_json::error::Error> = serde_json::from_str(&json_string);

    println!("{:?}", x);


    return;
    */
    let mut socket_addrs = HashMap::new();

    for i in 0..15 {
        let val = 12345 + i;
        socket_addrs.insert(i+1, format!("127.0.0.1:{}", val).parse().unwrap());

        //println!("{:?}", socket_addrs.get(&(i+1)));
    }


    let args: Vec<String> = env::args().collect();

    let id = &args[1];
    let id = id.parse::<i32>().unwrap();

    let node: Node<String> = Node::new(id, socket_addrs).unwrap();
    let node = Arc::new(node);

    let recv_thread_node = Arc::clone(&node);
    let recv_thread_handle = thread::spawn(move || {
        recv_thread_node.recv_loop();
    });

    let client_op_thread_node = Arc::clone(&node);
    let client_op_thread_handle = thread::spawn(move || {
        if id == 1 {
            let mut i = 0;

            loop {
                i += 1;
                client_op_thread_node.write(format!("{}", &i));

                if i % 1000 == 0 {
                    println!("{}", i);
                }
            }


        }
    });

    recv_thread_handle.join().unwrap();
    client_op_thread_handle.join().unwrap();





}
    /*
    return;
    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
    let socket = Arc::new(socket);
    let ts = Arc::new(Mutex::new(0));

    let recv_thread_ts = Arc::clone(&ts);
    let recv_thread_socket = Arc::clone(&socket);
    let recv_thread_handle = thread::spawn(move || {
        recv_loop(recv_thread_socket, recv_thread_ts);
    });

    let print_thread_ts = Arc::clone(&ts);
    let print_thread_handle = thread::spawn(move || {
        loop {
            let ts_d = ts.lock().unwrap();
            println!("{}", ts_d);
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    let send_thread_socket = Arc::clone(&socket);
    let send_thread_handle = thread::spawn(move || {
        send_loop(send_thread_socket);
    });

    recv_thread_handle.join().unwrap();
    print_thread_handle.join().unwrap();
    send_thread_handle.join().unwrap();

    
}

fn recv_loop(socket: Arc<UdpSocket>, ts: Arc<Mutex<i32>>) {
    loop {
        let mut buf = [0; 128];

        let (amt, src_addr) = socket.recv_from(&mut buf).unwrap();
        let string = str::from_utf8(&buf[0..amt]).unwrap();
        
        println!("Fick {} från {:?}", string, src_addr);

        let mut ts_d = ts.lock().unwrap();
        *ts_d += 1;
    }
}

fn send_loop(socket: Arc<UdpSocket>) {
    loop {
        let buf = "hej".as_bytes();
        let dst = "127.0.0.1:12345";

        socket.send_to(buf, &dst).unwrap();

        thread::sleep(time::Duration::from_millis(50));
    }
}

*/

