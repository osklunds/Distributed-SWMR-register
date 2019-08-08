
#[macro_use]
extern crate lazy_static;

mod arguments;

use std::process::Child;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use colored::Color;
use colored::Color::*;

use commons::execution;
use commons::node_info::NodeId;

use crate::arguments::ARGUMENTS;


fn main() {
    create_hosts_file();
    build_application();
    run_application();
}

fn create_hosts_file() {
    let hosts_file_string = hosts_file_string();
    let file_path = Path::new("hosts.txt");
    if file_path.exists() {
        if let Ok(existing_string) = fs::read_to_string(file_path) {
            if existing_string == hosts_file_string {
                return;
            }
        }

        fs::remove_file(file_path).expect("Could not remove existing hosts.txt file");
    }

    fs::write(file_path, hosts_file_string).expect("Could not write new hosts.txt file.");
}

fn hosts_file_string() -> String {
    let mut string = String::new();
    let port_offset = 62000;

    for node_id in 1..ARGUMENTS.number_of_nodes+1 {
        string.push_str(&format!("{},127.0.0.1:{}\n", node_id, node_id + port_offset));
    }

    string
}

fn build_application() {
    let command = format!("cargo build {} --manifest-path ../application/Cargo.toml", ARGUMENTS.release_mode_string);
    let mut build_process = execution::execute_local_command(&command);
    build_process.wait().unwrap();
}

fn run_application() {
    let mut run_processes = Vec::new();
    for node_id in 1..ARGUMENTS.number_of_nodes+1 {
        let run_process = run_single_application_instance(node_id);
        run_processes.push(run_process);
    }
 
    for run_process in run_processes.iter_mut() {
        run_process.wait().unwrap();
    }
}

fn run_single_application_instance(node_id: NodeId) -> Child {
    let color = color_from_node_id(node_id);
    let write_string = match node_id <= ARGUMENTS.number_of_writers {
        true  => "--write",
        false => ""
    };
    let read_string = match node_id <= ARGUMENTS.number_of_readers {
        true => "--read",
        false => ""
    };

    let command = format!("cargo run {} --manifest-path ../application/Cargo.toml -- {} hosts.txt {} {:?} {} {} {} {}", 
        ARGUMENTS.release_mode_string, node_id, 
        ARGUMENTS.run_length_string, color, 
        ARGUMENTS.print_client_operations_string, 
        ARGUMENTS.record_evaluation_info_string, 
        write_string, 
        read_string);

    execution::execute_local_command(&command)
}

fn color_from_node_id(node_id: NodeId) -> Color {
    let colors = vec![Black, Red, Green, Yellow, Blue, Magenta, Cyan];
    colors[(node_id as usize) % 7]
}