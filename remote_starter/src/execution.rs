
use std::process::{Command, Child};

use crate::arguments::NodeInfo;


pub fn execute_remote_command(command: &str, node_info: &NodeInfo) -> Child {
    let ssh_command = format!("ssh -i {} {}@{} {}", node_info.key_path, node_info.username, node_info.ip_addr_string(), command);

    execute_local_command(&ssh_command)
}

pub fn execute_scp_copy_of_path_relative_to_application_directory(path: &str, node_info: &NodeInfo) -> Child {
    let scp_command = format!("scp -i {} -r ../application/{} {}@{}:distributed_swmr_registers_remote_directory/{}", node_info.key_path, path, node_info.username, node_info.ip_addr_string(), path);

    execute_local_command(&scp_command)
}

pub fn execute_scp_copy_of_path_relative_to_remote_starter_directory(path: &str, node_info: &NodeInfo) -> Child {
    let scp_command = format!("scp -i {} -r {} {}@{}:distributed_swmr_registers_remote_directory/{}", node_info.key_path, path, node_info.username, node_info.ip_addr_string(), path);

    execute_local_command(&scp_command)
}

pub fn execute_local_command(command: &str) -> Child {
    Command::new("/bin/bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect(&format!("Failed to execute the command: {}", command))
}
