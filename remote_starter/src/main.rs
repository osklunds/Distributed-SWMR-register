
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;

mod arguments;

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
    



    /*
    let mut install_processes = Vec::new();
    for node_info in node_infos.iter() {
        let install_process = execute_remote_command("\"curl https://sh.rustup.rs -sSf > rustup.sh;sh rustup.sh -y\"", &node_info);
        install_processes.push(install_process);
    }

    for install_process in install_processes.iter_mut() {
        install_process.wait().unwrap();
    }
    */

    
    for node_info in node_infos.iter() {
        execute_remote_command("rm -r distributed_swmr_registers_remote_directory/", &node_info).wait().unwrap();
        execute_remote_command("mkdir distributed_swmr_registers_remote_directory/", &node_info).wait().unwrap();

        execute_scp_copy_of_application_path("src/", &node_info).wait().unwrap();
        execute_scp_copy_of_application_path("Cargo.toml", &node_info).wait().unwrap();
        execute_scp_copy_of_application_path("Cargo.lock", &node_info).wait().unwrap();
        execute_scp_copy_of_remote_starter_path("hosts.txt", &node_info).wait().unwrap();
    }

    

    let mut build_processes = Vec::new();
    for node_info in node_infos.iter() {
        let build_process = execute_remote_command("\"cd distributed_swmr_registers_remote_directory/;../.cargo/bin/cargo build;cd ..\"", &node_info);
        build_processes.push(build_process);
    }
    for build_process in build_processes.iter_mut() {
        build_process.wait().unwrap();
    }

    let mut run_processes = Vec::new();
    for node_info in node_infos.iter() {
        let write_string = match node_info.node_id <= number_of_writers {
            true  => "--write",
            false => ""
        };
        let read_string = match node_info.node_id <= number_of_readers {
            true => "--read",
            false => ""
        };

        let command_string = format!("\"cd distributed_swmr_registers_remote_directory/;../.cargo/bin/cargo run {} -- {} hosts.txt {} {} {} {} {};cd..\"", 
            release_mode_string, 
            node_info.node_id, 
            run_length,
            record_evaluation_info_string,
            print_client_operations_string,
            write_string, 
            read_string);

        println!("{}", command_string);

        let run_process = execute_remote_command(&command_string, &node_info);

        run_processes.push(run_process);
    }
    for run_process in run_processes.iter_mut() {
        run_process.wait().unwrap();
    }

}


fn execute_command(command: &str) -> Child {
    Command::new("/bin/bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect(&format!("Failed to execute the command: {}", command))
}

fn execute_remote_command(command: &str, node_info: &NodeInfo) -> Child {
    let ssh_command = format!("ssh -i {} {}@{} {}", node_info.key_path, node_info.username, node_info.ip_addr_string(), command);

    execute_command(&ssh_command)
}

fn execute_scp_copy_of_application_path(path: &str, node_info: &NodeInfo) -> Child {
    let scp_command = format!("scp -i {} -r ../application/{} {}@{}:distributed_swmr_registers_remote_directory/{}", node_info.key_path, path, node_info.username, node_info.ip_addr_string(), path);

    execute_command(&scp_command)
}

fn execute_scp_copy_of_remote_starter_path(path: &str, node_info: &NodeInfo) -> Child {
    let scp_command = format!("scp -i {} -r {} {}@{}:distributed_swmr_registers_remote_directory/{}", node_info.key_path, path, node_info.username, node_info.ip_addr_string(), path);

    execute_command(&scp_command)
}

