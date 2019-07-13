
//#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

mod entry;
mod register;
mod abd_node;
mod messages;
mod settings;
mod terminal_output;
mod communicator;
mod mediator;
mod responsible_cell;

use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc::{self, TryRecvError};


use settings::SETTINGS;
use terminal_output::printlnu;
use mediator::Mediator;


fn main() {
    SETTINGS.node_id();

    let mediator = Mediator::new();
    
    // This is important when running locally. If some application
    // processes start before all have been built, they will
    // consume so much CPU time that the build processes
    // are very slow, and hence some nodes will be run for
    // a longer time than others.
    thread::sleep(Duration::from_millis(100 * SETTINGS.number_of_nodes() as u64));

    let (read_tx, read_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_thread_mediator = Arc::clone(&mediator);
    let read_thread_handle = thread::spawn(move || {
        if SETTINGS.node_id() == 1 {
            let mut i = 0;
            loop {
                i += 1;

                //printlnu(format!("Start read {}", i));
                let _res = read_thread_mediator.read_all();
                //printlnu(format!("Stop read {}\n{}", i, res));

                match read_rx.try_recv() {
                    Err(TryRecvError::Empty) => {},
                    _                        => break
                }
            }
        }
    });

    let write_thread_mediator = Arc::clone(&mediator);
    let write_thread_handle = thread::spawn(move || {
        if SETTINGS.node_id() != 1 {
            let mut i = 0;
            loop {
                i += 1;

                //printlnu(format!("Start write {}", i));
                write_thread_mediator.write("".to_string());
                //printlnu(format!("Stop write {}", i));

                match write_rx.try_recv() {
                    Err(TryRecvError::Empty) => {},
                    _                        => break
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(10));

    let _ = read_tx.send(());
    let _ = write_tx.send(());

    read_thread_handle.join().unwrap();
    write_thread_handle.join().unwrap();



    // If a node doesn't read or write, we let it sleep forever
    // so that it still sends ack messages.
    loop {
        thread::sleep(Duration::from_millis(100000));
    }
}
