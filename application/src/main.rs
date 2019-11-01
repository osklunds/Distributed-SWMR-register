#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

mod abd_node;
mod communicator;
mod configuration_manager;
mod mediator;
mod messages;
mod quorum;
mod responsible_cell;
mod settings;
mod terminal_output;

use std::fs;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use commons::arguments;
use commons::types::Int;

use crate::mediator::{Med, Mediator, MediatorImpl};
use crate::settings::SETTINGS;
use crate::terminal_output::printlnu;


fn main() {
    SETTINGS.node_id();

    let mediator = MediatorImpl::new();

    // This is important when running locally. If some application
    // processes start before all have been built, they will
    // consume so much CPU time that the build processes
    // are very slow, and hence some nodes will be run for
    // a longer time than others.
    thread::sleep(Duration::from_millis(
        100 * SETTINGS.number_of_nodes() as u64,
    ));

    let (read_tx, write_tx) =
        start_client_threads_and_get_channel_send_ends(&mediator);

    sleep_time_specified_by_arguments();

    let _ = read_tx.send(());
    let _ = write_tx.send(());

    let mut run_result = mediator.run_result();

    run_result.metadata.node_id = SETTINGS.node_id();
    run_result.metadata.is_reader = SETTINGS.should_read();
    run_result.metadata.is_writer = SETTINGS.should_write();
    run_result.metadata.run_length =
        SETTINGS.run_length().as_secs() as Int;

    let json = serde_json::to_string(&*run_result).unwrap();
    printlnu(format!("{}", &json));
    fs::write(
        arguments::run_result_file_name_from_node_id(
            SETTINGS.node_id(),
        ),
        json,
    )
    .expect("Could not write the json result file");
}

fn start_client_threads_and_get_channel_send_ends<M: Med>(
    mediator: &Arc<M>,
) -> (Sender<()>, Sender<()>) {
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

fn client_reads<M: Med>(read_rx: Receiver<()>, mediator: Arc<M>) {
    let mut read_number = 0;
    loop {
        read_number += 1;

        if SETTINGS.print_client_operations() {
            printlnu(format!("Start read {}", read_number));
        }

        let res = mediator.read();

        if SETTINGS.print_client_operations() {
            printlnu(format!("Stop read {}. Result: {}", read_number, res));
        }

        match read_rx.try_recv() {
            Err(TryRecvError::Empty) => {}
            _ => break,
        }
    }
}

fn client_writes<M: Med>(write_rx: Receiver<()>, mediator: Arc<M>) {
    let mut write_number = 0;
    loop {
        write_number += 1;

        if SETTINGS.print_client_operations() {
            printlnu(format!("Start write {}", write_number));
        }

        mediator.write(format!("Val: {}", write_number));

        if SETTINGS.print_client_operations() {
            printlnu(format!("End write {}", write_number));
        }

        match write_rx.try_recv() {
            Err(TryRecvError::Empty) => {}
            _ => break,
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
