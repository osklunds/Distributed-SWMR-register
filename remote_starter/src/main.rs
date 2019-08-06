
//#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;

mod arguments;
mod execution;

use std::collections::HashSet;
use std::vec::Vec;

use colored::Color;
use colored::Color::*;
use ctrlc;

use crate::arguments::{ARGUMENTS, NodeId};


fn main() {
    stop_all_remote_processes();
    
    ctrlc::set_handler(move || {
        println!("I will now exit. But first I will stop all processes I have started on the remote computers.");
        stop_all_remote_processes();

    }).unwrap();

    if ARGUMENTS.install {
        install_rust_on_remote_computers();
        upload_source_code_and_hosts_file();
        build_source_code();
    } else {
        run_application_on_remote_computers();
    }
}

fn stop_all_remote_processes() {
    let mut stop_processes = Vec::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let command = "killall distributed_swmr_registers";
        let stop_process = execution::execute_remote_command(command, &node_info);
        stop_processes.push(stop_process);
    }

    for stop_process in stop_processes.iter_mut() {
        stop_process.wait().unwrap();
    }
}

fn install_rust_on_remote_computers() {
    let mut install_processes = Vec::new();
    let mut handled_ip_addrs = HashSet::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let ip_addr = node_info.ip_addr_string();
        if handled_ip_addrs.insert(ip_addr) {
            let install_process = execution::execute_remote_command("\"curl https://sh.rustup.rs -sSf > rustup.sh;sh rustup.sh -y\"", &node_info);
            install_processes.push(install_process);
        }
    }

    for install_process in install_processes.iter_mut() {
        install_process.wait().unwrap();
    }
}

fn upload_source_code_and_hosts_file() {
    let mut handled_ip_addrs = HashSet::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let ip_addr = node_info.ip_addr_string();
        if handled_ip_addrs.insert(ip_addr) {
            execution::execute_remote_command("rm -r distributed_swmr_registers_remote_directory/", &node_info).wait().unwrap();
            execution::execute_remote_command("mkdir distributed_swmr_registers_remote_directory/", &node_info).wait().unwrap();

            execution::execute_scp_copy_of_path_relative_to_application_directory("src/", &node_info).wait().unwrap();
            execution::execute_scp_copy_of_path_relative_to_application_directory("Cargo.toml", &node_info).wait().unwrap();
            execution::execute_scp_copy_of_path_relative_to_application_directory("Cargo.lock", &node_info).wait().unwrap();
            execution::execute_scp_copy_of_path_relative_to_remote_starter_directory("hosts.txt", &node_info).wait().unwrap();
        }
    }
}

fn build_source_code() {
    let mut build_processes = Vec::new();
    let mut handled_ip_addrs = HashSet::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let ip_addr = node_info.ip_addr_string();
        if handled_ip_addrs.insert(ip_addr) {
            let command = format!("\"cd distributed_swmr_registers_remote_directory/;../.cargo/bin/cargo build {};cd ..\"", ARGUMENTS.release_mode_string);
            let build_process = execution::execute_remote_command(&command, &node_info);
            build_processes.push(build_process);
        }
    }

    for build_process in build_processes.iter_mut() {
        build_process.wait().unwrap();
    }
}

fn run_application_on_remote_computers() {
    let mut run_processes = Vec::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let write_string = match node_info.node_id <= ARGUMENTS.number_of_writers {
            true  => "--write",
            false => ""
        };
        let read_string = match node_info.node_id <= ARGUMENTS.number_of_readers {
            true => "--read",
            false => ""
        };

        let command_string = format!("\"cd distributed_swmr_registers_remote_directory/;../.cargo/bin/cargo run {} -- {} hosts.txt {} {:?} {} {} {} {};cd ..\"", 
            ARGUMENTS.release_mode_string,
            node_info.node_id,
            ARGUMENTS.run_length_string,
            color_from_node_id(node_info.node_id),
            ARGUMENTS.record_evaluation_info_string,
            ARGUMENTS.print_client_operations_string,
            write_string, 
            read_string);

        let run_process = execution::execute_remote_command(&command_string, &node_info);

        run_processes.push(run_process);
    }

    for run_process in run_processes.iter_mut() {
        run_process.wait().unwrap();
    }
}

fn color_from_node_id(node_id: NodeId) -> Color {
    let colors = vec![Black, Red, Green, Yellow, Blue, Magenta, Cyan];
    colors[(node_id as usize) % 7]
}
