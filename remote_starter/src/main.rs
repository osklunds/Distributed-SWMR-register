//#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;

mod arguments;

use std::collections::HashSet;
use std::vec::Vec;

use ctrlc;

use commons::execution;
use commons::misc;
use commons::node_info::NodeInfo;
use commons::remote_machine::*;

use crate::arguments::ARGUMENTS;

fn main() {
    stop_all_remote_processes();

    ctrlc::set_handler(move || {
        println!("I will now exit. But first I will stop all processes I have started on the remote computers.");
        stop_all_remote_processes();

    }).expect("Could not set the CTRL+C handler.");

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
        stop_process
            .wait()
            .expect("Could not wait for a stop process.");
    }
}

fn install_rust_on_remote_computers() {
    let mut install_processes = Vec::new();
    let mut handled_ip_addrs = HashSet::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let ip_addr = node_info.ip_addr_string();
        if handled_ip_addrs.insert(ip_addr) {
            let install_process = execution::execute_remote_command(
                "\"curl https://sh.rustup.rs -sSf > rustup.sh;sh rustup.sh -y\"",
                &node_info,
            );
            install_processes.push(install_process);
        }
    }

    for install_process in install_processes.iter_mut() {
        install_process
            .wait()
            .expect("Could not wait for an install process.");
    }
}

fn upload_source_code_and_hosts_file() {
    let mut handled_ip_addrs = HashSet::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let ip_addr = node_info.ip_addr_string();
        if handled_ip_addrs.insert(ip_addr) {
            execution::execute_remote_command(
                &format!("mkdir {}/", REMOTE_DIRECTORY_NAME),
                &node_info,
            )
            .wait()
            .expect("Could not wait for a folder creation command.");
            update_crate_source_on_remote("application", &node_info);
            update_crate_source_on_remote("commons", &node_info);
            execution::scp_copy_of_local_source_path_to_remote_destination_path(
                &ARGUMENTS.hosts_file,
                &format!("application/{}", REMOTE_HOSTS_FILE_NAME),
                &node_info,
            )
            .wait()
            .expect("Could not wait for the hosts file copy command.");
        }
    }
}

fn update_crate_source_on_remote(crate_name: &str, node_info: &NodeInfo) {
    execution::execute_remote_command(
        &format!("rm -r {}/{}/src/", REMOTE_DIRECTORY_NAME, crate_name),
        &node_info,
    )
    .wait()
    .expect("Could not wait for a remote command.");
    execution::execute_remote_command(
        &format!("mkdir {}/{}", REMOTE_DIRECTORY_NAME, crate_name),
        &node_info,
    )
    .wait()
    .expect("Could not wait for a remote command.");

    copy_path_from_local_crate_to_remote_crate("src/", crate_name, node_info);
    copy_path_from_local_crate_to_remote_crate("Cargo.toml", crate_name, node_info);
    copy_path_from_local_crate_to_remote_crate("Cargo.lock", crate_name, node_info);
}

fn copy_path_from_local_crate_to_remote_crate(path: &str, crate_name: &str, node_info: &NodeInfo) {
    execution::scp_copy_of_local_source_path_to_remote_destination_path(
        &format!("../{}/{}", crate_name, path),
        &format!("{}/{}", crate_name, path),
        &node_info,
    )
    .wait()
    .expect("Could not wait for a remote copy.");
}

fn build_source_code() {
    let mut build_processes = Vec::new();
    let mut handled_ip_addrs = HashSet::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let ip_addr = node_info.ip_addr_string();
        if handled_ip_addrs.insert(ip_addr) {
            let command = format!(
                "\"cd {}/application/;../../.cargo/bin/cargo build {};cd ../../\"",
                REMOTE_DIRECTORY_NAME, ARGUMENTS.release_mode_string
            );
            let build_process = execution::execute_remote_command(&command, &node_info);
            build_processes.push(build_process);
        }
    }

    for build_process in build_processes.iter_mut() {
        build_process
            .wait()
            .expect("Could not wait for a build process.");
    }
}

fn run_application_on_remote_computers() {
    let mut run_processes = Vec::new();

    for node_info in ARGUMENTS.node_infos.iter() {
        let write_string = match node_info.node_id <= ARGUMENTS.number_of_writers {
            true => "--write",
            false => "",
        };
        let read_string = match node_info.node_id <= ARGUMENTS.number_of_readers {
            true => "--read",
            false => "",
        };

        let command_string = format!("\"cd {}/application/;../../.cargo/bin/cargo run {} -- {} {} -l {} -c {:?} {} {} {} {};cd ../../\"",
            REMOTE_DIRECTORY_NAME,
            ARGUMENTS.release_mode_string,
            node_info.node_id,
            REMOTE_HOSTS_FILE_NAME,
            ARGUMENTS.run_length_string,
            misc::color_from_node_id(node_info.node_id),
            ARGUMENTS.record_evaluation_info_string,
            ARGUMENTS.print_client_operations_string,
            write_string,
            read_string);

        let run_process = execution::execute_remote_command(&command_string, &node_info);

        run_processes.push(run_process);
    }

    for run_process in run_processes.iter_mut() {
        run_process
            .wait()
            .expect("Could not wait for a run process.");
    }
}
