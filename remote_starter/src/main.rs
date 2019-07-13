
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;

mod arguments;
mod execution;

use std::collections::{HashMap, HashSet};
//use std::iter::FromIterator;
use std::net::SocketAddr;
use std::net::IpAddr::V4;
use std::net::ToSocketAddrs;

use std::process::{Command, Child};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

use crate::arguments::{ARGUMENTS, Arguments, NodeInfo};


fn main() {
    if ARGUMENTS.full_install {
        install_rust_on_remote_computers();
        upload_source_code_and_hosts_file();
        build_source_code();
    }

    run_application_on_remote_computers();
}

fn install_rust_on_remote_computers() {
    let mut install_processes = Vec::new();
    for node_info in ARGUMENTS.node_infos.iter() {
        let install_process = execution::execute_remote_command("\"curl https://sh.rustup.rs -sSf > rustup.sh;sh rustup.sh -y\"", &node_info);
        install_processes.push(install_process);
    }

    for install_process in install_processes.iter_mut() {
        install_process.wait().unwrap();
    }
}

fn upload_source_code_and_hosts_file() {
    for node_info in ARGUMENTS.node_infos.iter() {
        execution::execute_remote_command("rm -r distributed_swmr_registers_remote_directory/", &node_info).wait().unwrap();
        execution::execute_remote_command("mkdir distributed_swmr_registers_remote_directory/", &node_info).wait().unwrap();

        execution::execute_scp_copy_of_path_relative_to_application_directory("src/", &node_info).wait().unwrap();
        execution::execute_scp_copy_of_path_relative_to_application_directory("Cargo.toml", &node_info).wait().unwrap();
        execution::execute_scp_copy_of_path_relative_to_application_directory("Cargo.lock", &node_info).wait().unwrap();
        execution::execute_scp_copy_of_path_relative_to_remote_starter_directory("hosts.txt", &node_info).wait().unwrap();
    }
}

fn build_source_code() {
    let mut build_processes = Vec::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let build_process = execution::execute_remote_command("\"cd distributed_swmr_registers_remote_directory/;../.cargo/bin/cargo build;cd ..\"", &node_info);
        build_processes.push(build_process);
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

        let command_string = format!("\"cd distributed_swmr_registers_remote_directory/;../.cargo/bin/cargo run {} -- {} hosts.txt {} {} {} {} {};cd..\"", 
            ARGUMENTS.release_mode_string, 
            node_info.node_id, 
            ARGUMENTS.run_length_string,
            ARGUMENTS.record_evaluation_info_string,
            ARGUMENTS.print_client_operations_string,
            write_string, 
            read_string);

        println!("{}", command_string);

        let run_process = execution::execute_remote_command(&command_string, &node_info);

        run_processes.push(run_process);
    }
    for run_process in run_processes.iter_mut() {
        run_process.wait().unwrap();
    }
}






