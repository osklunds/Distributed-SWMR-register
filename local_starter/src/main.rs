#[macro_use]
extern crate lazy_static;

mod arguments;

use std::fs;
use std::path::Path;
use std::process::Child;
use std::vec::Vec;

use commons::execution;
use commons::types::NodeId;

use crate::arguments::ARGUMENTS;

fn main() {
    check_write_read_soundness();
    create_hosts_file();
    build_application();
    run_application();
}

fn check_write_read_soundness() {
    if ARGUMENTS.should_write && ARGUMENTS.number_of_readers >= ARGUMENTS.number_of_nodes {
        panic!("If the writer node shall write, the number of readers must be less than the number of nodes in total. The writer cannot read and write at the same time.");
    }
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

        fs::remove_file(file_path)
            .expect("Could not remove existing hosts.txt file");
    }

    fs::write(file_path, hosts_file_string)
        .expect("Could not write the new hosts.txt file.");
}

fn hosts_file_string() -> String {
    let mut string = String::new();
    let port_offset = 62000;

    for node_id in 1..ARGUMENTS.number_of_nodes + 1 {
        string.push_str(&format!(
            "{},127.0.0.1:{}\n",
            node_id,
            node_id + port_offset
        ));
    }

    string
}

fn build_application() {
    let command = format!(
        "cargo build {} --manifest-path ../application/Cargo.toml",
        ARGUMENTS.release_mode_string
    );
    let mut build_process = execution::execute_local_command(&command);
    build_process
        .wait()
        .expect("Could not wait for the build process.");
}

fn run_application() {
    let mut run_processes = Vec::new();
    for node_id in 1..ARGUMENTS.number_of_nodes + 1 {
        let run_process = run_single_application_instance(node_id);
        run_processes.push(run_process);
    }

    for run_process in run_processes.iter_mut() {
        run_process
            .wait()
            .expect("Could not wait for the run process.");
    }
}

fn run_single_application_instance(node_id: NodeId) -> Child {
    let color = commons::arguments::color_from_node_id(node_id);
    let write_string = if node_id == ARGUMENTS.number_of_nodes && ARGUMENTS.should_write {
        "--write"
    } else {
        ""
    };
    let read_string = match node_id <= ARGUMENTS.number_of_readers {
        true => "--read",
        false => "",
    };

    let command = format!("cargo run {} --manifest-path ../application/Cargo.toml -- {} hosts.txt -c {:?} -l {} {} {} {}", 
        ARGUMENTS.release_mode_string,
        node_id,
        color,
        ARGUMENTS.run_length_string,
        ARGUMENTS.print_client_operations_string,
        write_string,
        read_string);

    execution::execute_local_command(&command)
}
