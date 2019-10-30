//#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;

mod arguments;

use std::collections::HashSet;
use std::process::Child;
use std::thread;
use std::vec::Vec;

use ctrlc;

use commons::execution;
use commons::node_info::NodeInfo;
use commons::remote_machine::*;

use crate::arguments::ARGUMENTS;

fn main() {
    stop_all_remote_processes();
    set_ctrl_c_handler();

    if ARGUMENTS.install {
        run_install_script_on_remote_computers();
        upload_source_code_and_hosts_file();
        build_source_code();
    } else {
        run_application_on_remote_computers();
    }
}

fn stop_all_remote_processes() {
    run_function_on_each_unique_host_in_parallell(&stop_remote_processes);
}

fn run_function_on_each_unique_host_in_parallell(
    function_to_run: &Fn(&NodeInfo) -> Child,
) {
    run_function_on_hosts_in_parallell(
        function_to_run,
        &ARGUMENTS.node_infos_for_unique_hosts(),
    );
}

fn run_function_on_hosts_in_parallell(
    function_to_run: &Fn(&NodeInfo) -> Child,
    hosts: &HashSet<NodeInfo>,
) {
    let mut processes = Vec::new();

    for node_info in hosts.iter() {
        let process = function_to_run(&node_info);
        processes.push(process);
    }

    for process in processes.iter_mut() {
        process.wait().expect("A parallelly run process failed.");
    }
}

fn stop_remote_processes(node_info: &NodeInfo) -> Child {
    let stop_command = format!("pkill -u {}", node_info.username);
    execution::execute_remote_command(&stop_command, node_info)
}

fn set_ctrl_c_handler() {
    ctrlc::set_handler(move || {
        println!("I will now exit. But first I will stop all processes I have started on the remote computers.");
        stop_all_remote_processes();

    }).expect("Could not set the CTRL+C handler.");
}

fn run_install_script_on_remote_computers() {
    run_function_on_each_unique_host_in_parallell(
        &run_install_script_on_remote_computer,
    );
}

fn run_install_script_on_remote_computer(node_info: &NodeInfo) -> Child {
    create_remote_directory(node_info);
    copy_install_script(node_info);
    run_install_script(node_info)
}

fn create_remote_directory(node_info: &NodeInfo) {
    execution::execute_remote_command(
        &format!("mkdir {}/", REMOTE_DIRECTORY_NAME),
        node_info,
    )
    .wait()
    .expect("mkdir failed.");
}

fn copy_install_script(node_info: &NodeInfo) {
    execution::scp_copy_of_local_source_path_to_remote_destination_path(
        &node_info.script_path,
        REMOTE_INSTALL_SCRIPT_NAME,
        node_info,
    )
    .wait()
    .expect("script upload failed.");
}

fn run_install_script(node_info: &NodeInfo) -> Child {
    execution::execute_remote_command(
        &format!(
            "{}/{}",
            REMOTE_DIRECTORY_NAME, REMOTE_INSTALL_SCRIPT_NAME
        ),
        node_info,
    )
}

fn upload_source_code_and_hosts_file() {
    let mut join_handles = Vec::new();

    for node_info in ARGUMENTS.node_infos_for_unique_hosts().iter() {
        let node_info_thread = node_info.clone();
        let join_handle = thread::spawn(move || {
            upload_source_code_and_hosts_file_to_single_remote_computer(
                &node_info_thread,
            );
        });

        join_handles.push(join_handle);
    }

    for join_handle in join_handles.into_iter() {
        join_handle
            .join()
            .expect("Could not join a thread for source code upload.");
    }
}

fn upload_source_code_and_hosts_file_to_single_remote_computer(
    node_info: &NodeInfo,
) {
    update_crate_source_on_remote("application", node_info);
    update_crate_source_on_remote("commons", node_info);
    upload_hosts_file(node_info);
}

fn update_crate_source_on_remote(crate_name: &str, node_info: &NodeInfo) {
    delete_src_folder_of_crate(crate_name, node_info);
    create_crate_folder(crate_name, node_info);

    copy_path_from_local_crate_to_remote_crate(
        "src/", crate_name, node_info,
    );
    copy_path_from_local_crate_to_remote_crate(
        "Cargo.toml",
        crate_name,
        node_info,
    );
    copy_path_from_local_crate_to_remote_crate(
        "Cargo.lock",
        crate_name,
        node_info,
    );
}

fn delete_src_folder_of_crate(crate_name: &str, node_info: &NodeInfo) {
    execution::execute_remote_command(
        &format!("rm -r {}/{}/src/", REMOTE_DIRECTORY_NAME, crate_name),
        &node_info,
    )
    .wait()
    .expect("Delete src folder process failed.");
}

fn create_crate_folder(crate_name: &str, node_info: &NodeInfo) {
    execution::execute_remote_command(
        &format!("mkdir {}/{}", REMOTE_DIRECTORY_NAME, crate_name),
        &node_info,
    )
    .wait()
    .expect("Could not wait for a remote command.");
}

fn copy_path_from_local_crate_to_remote_crate(
    path: &str,
    crate_name: &str,
    node_info: &NodeInfo,
) {
    execution::scp_copy_of_local_source_path_to_remote_destination_path(
        &format!("../{}/{}", crate_name, path),
        &format!("{}/{}", crate_name, path),
        &node_info,
    )
    .wait()
    .expect("Could not wait for a remote copy.");
}

fn upload_hosts_file(node_info: &NodeInfo) {
    execution::scp_copy_of_local_source_path_to_remote_destination_path(
        &ARGUMENTS.hosts_file,
        &format!("application/{}", REMOTE_HOSTS_FILE_NAME),
        node_info,
    )
    .wait()
    .expect("Could not wait for the hosts file copy command.");
}

fn build_source_code() {
    run_function_on_each_unique_host_in_parallell(
        &build_source_code_on_remote_computer,
    );
}

fn build_source_code_on_remote_computer(node_info: &NodeInfo) -> Child {
    let command = format!(
        "\"cd {}/application/;cargo build {};\"",
        REMOTE_DIRECTORY_NAME, ARGUMENTS.release_mode_string
    );
    execution::execute_remote_command(&command, &node_info)
}

fn run_application_on_remote_computers() {
    run_function_on_all_hosts_in_parallell(
        &run_application_on_remote_computer,
    );
}

fn run_function_on_all_hosts_in_parallell(
    function_to_run: &Fn(&NodeInfo) -> Child,
) {
    run_function_on_hosts_in_parallell(
        function_to_run,
        &ARGUMENTS.node_infos,
    );
}

fn run_application_on_remote_computer(node_info: &NodeInfo) -> Child {
    let write_string =
        match node_info.node_id <= ARGUMENTS.number_of_writers {
            true => "--write",
            false => "",
        };
    let read_string =
        match node_info.node_id <= ARGUMENTS.number_of_readers {
            true => "--read",
            false => "",
        };

    let command_string = format!(
        "\"cd {}/application/;cargo run {} -- {} {} -l {} -c {:?} {} {} {} {};cd ../../\"",
        REMOTE_DIRECTORY_NAME,
        ARGUMENTS.release_mode_string,
        node_info.node_id,
        REMOTE_HOSTS_FILE_NAME,
        ARGUMENTS.run_length_string,
        commons::arguments::color_from_node_id(node_info.node_id),
        ARGUMENTS.record_evaluation_info_string,
        ARGUMENTS.print_client_operations_string,
        write_string,
        read_string
    );

    execution::execute_remote_command(&command_string, &node_info)
}
