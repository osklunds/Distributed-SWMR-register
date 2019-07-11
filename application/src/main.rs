
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

use std::time::SystemTime;
use std::time::Duration;
use std::sync::Arc;
use std::thread;

use settings::SETTINGS;
use terminal_output::printlnu;
use mediator::Mediator;


fn main() {
    SETTINGS.node_id();
    let mediator = Mediator::new();
    
    let write_thread_mediator = Arc::clone(&mediator);
    let write_thread_handle = thread::spawn(move || {
        if write_thread_mediator.node_id() == 1 {
            thread::sleep(Duration::from_millis(500));

            let start = SystemTime::now();

            for _ in 0..1000 {
                write_thread_mediator.write(format!("Hej"));
                //printlnu(format!("{}", i));
            }

            let elapsed = start.elapsed().unwrap();
            printlnu(format!("{:?}", elapsed));
        }
    });
    write_thread_handle.join().unwrap();

    loop {
        thread::sleep(Duration::from_millis(1000000));
    }
}
