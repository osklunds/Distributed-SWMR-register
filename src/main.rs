
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use std::net::UdpSocket;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::str;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;

use std::default::Default;

fn main() {



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

struct Node<V> {
    id: Arc<i32>,
    ts: Arc<Mutex<i32>>,
    reg: Arc<Mutex<Register<V>>>,
    socket: Arc<UdpSocket>
}

impl<V: Default> Node<V> {
    pub fn new(node_id: i32, node_ids: HashSet<i32>) -> Node<V> {
        let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

        Node {
            id: Arc::new(node_id),
            ts: Arc::new(Mutex::new(-1)),
            reg: Arc::new(Mutex::new(Register::new(node_ids))),
            socket: Arc::new(socket)
        }
    }
}

struct Register<V> {
    map: HashMap<i32, Entry<V>>
}

impl<V: Default> Register<V> {
    pub fn new(node_ids: HashSet<i32>) -> Register<V> {
        let mut map = HashMap::new();
        for node_id in node_ids {
            map.insert(node_id, Entry::new(-1, V::default()));
        }

        Register {
            map: map
        }
    }

    pub fn get(self: &Self, node_id: i32) -> Option<&Entry<V>> {
        self.map.get(&node_id)
    }

    pub fn set(self: &mut Self, node_id: i32, entry: Entry<V>) {
        self.map.insert(node_id, entry);
    }
}

impl<V: Display> Display for Register<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for node_id in self.map.keys() {
            let entry = &self.map.get(node_id).unwrap();
            string.push_str(&format!("{}: {}", node_id, entry));
        }

        write!(f, "{}", string)
    }
}

impl<V: PartialEq> PartialEq for Register<V> {
    fn eq(&self, other: &Self) -> bool {
        for node_id in self.map.keys() {
            let my_val = self.map.get(&node_id);
            let other_val = other.map.get(&node_id);

            if my_val != other_val {
                return false;
            }
        }
        return true;
    }
}

impl<V: PartialOrd> PartialOrd for Register<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if less_than_or_equal(&self.map, &other.map) {
            Some(Ordering::Less)
        } else if less_than_or_equal(&other.map, &self.map) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

fn less_than_or_equal<V: PartialOrd>(lhs: &HashMap<i32, V>, rhs: &HashMap<i32, V>) -> bool {
    for node_id in lhs.keys() {
        let lhs_val = lhs.get(&node_id).unwrap();
        let rhs_val = rhs.get(&node_id).expect("Attempting to compare registers with different keys.");

        if lhs_val > rhs_val {
            return false;
        }
    }
    return true;
}

pub struct Entry<V> {
    pub ts: i32,
    pub val: V
}

impl<V> Entry<V> {
    fn new(ts: i32, val: V) -> Entry<V> {
        Entry {
            ts: ts,
            val: val
        }
    }
}

impl<V: Display> Display for Entry<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ts = {}, val = {}]", self.ts, self.val)
    }
}

impl<V> PartialEq for Entry<V> {
    fn eq(&self, other: &Self) -> bool {
        self.ts == other.ts
    }
}

impl<V> PartialOrd for Entry<V> {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        self.ts.partial_cmp(&other.ts)
    }
}
