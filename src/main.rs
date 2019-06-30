
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]


use std::str;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use std::collections::{HashMap, HashSet};




use std::default::Default;

mod register;
mod node;
mod config_manager;

fn main() {

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

