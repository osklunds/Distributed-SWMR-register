
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
    
    thread::sleep(Duration::from_millis(100 * SETTINGS.number_of_nodes() as u64));
    /*
    let read_thread_mediator = Arc::clone(&mediator);
    let read_thread_handle = thread::spawn(move || {
        if SETTINGS.node_id() == 1 {
            printlnu(format!("I am going to read {}", SETTINGS.node_id()));
            let mut i = 0;
            loop {
                i += 1;
                printlnu(format!("Start read {}", i));
                let res = read_thread_mediator.read_all();
                printlnu(format!("Stop read {}\n{}", i, res));
            }
        }
    });
    */

    let write_thread_mediator = Arc::clone(&mediator);
    let write_thread_handle = thread::spawn(move || {
        if SETTINGS.node_id() == 1 || true {
            let start = SystemTime::now();
            let mut i = 0;
            loop {
                i += 1;
                //printlnu(format!("Start write {}", i));
                write_thread_mediator.write("".to_string());
                //printlnu(format!("Stop write {}", i));

                if i == 10000 {
                    break;
                }
            }

            let elapsed = start.elapsed();
            printlnu(format!("{:?}", elapsed));
        }
    });





    //read_thread_handle.join().unwrap();
    write_thread_handle.join().unwrap();

    loop {
        thread::sleep(Duration::from_millis(100000));
    }
}
