
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

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
use std::sync::mpsc::{self, TryRecvError, Receiver, Sender};


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

    let (read_tx, write_tx) = start_client_threads_and_get_channel_send_ends(&mediator);

    sleep_time_specified_by_arguments();

    let _ = read_tx.send(());
    let _ = write_tx.send(());
}

fn start_client_threads_and_get_channel_send_ends(mediator: &Arc<Mediator>) -> (Sender<()>, Sender<()>) {
    let (read_tx, read_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_thread_mediator = Arc::clone(mediator);
    thread::spawn(move || {
        if SETTINGS.should_read() {
            client_reads(read_rx, read_thread_mediator);
        }
    });
    
    let write_thread_mediator = Arc::clone(mediator);
    thread::spawn(move || {
        if SETTINGS.should_write() {
            client_writes(write_rx, write_thread_mediator);
        }
    });

    (read_tx, write_tx)
}

fn client_reads(read_rx: Receiver<()>, mediator: Arc<Mediator>) {
    if SETTINGS.should_read() {
        let mut read_number = 0;
        loop {
            read_number += 1;

            if SETTINGS.print_client_operations() {
                printlnu(format!("Start read {}", read_number));
            }

            let res = mediator.read_all();
            
            if SETTINGS.print_client_operations() {
                printlnu(format!("Stop read {}\n{}", read_number, res));
            }               

            match read_rx.try_recv() {
                Err(TryRecvError::Empty) => {},
                _                        => break
            }
        }
    }
}

fn client_writes(write_rx: Receiver<()>, mediator: Arc<Mediator>) {
    let mut write_number = 0;
    loop {
        write_number += 1;

        if SETTINGS.print_client_operations() {
            printlnu(format!("Start write {}", write_number));
        }

        mediator.write("".to_string());

        if SETTINGS.print_client_operations() {
            printlnu(format!("End write {}", write_number));
        }

        match write_rx.try_recv() {
            Err(TryRecvError::Empty) => {},
            _                        => break
        }
    }
}

fn sleep_time_specified_by_arguments() {
    if SETTINGS.run_length() == Duration::from_secs(0) {
        loop {
            thread::sleep(Duration::from_secs(60));
        }
    } else {
        thread::sleep(SETTINGS.run_length());
    }
}
