
use std::process::{Command, Child};


use crate::node_info::NodeInfo;


pub fn execute_remote_command(command: &str, node_info: &NodeInfo) -> Child {
    let ssh_command = format!("ssh -i {} {}@{} {}", node_info.key_path, node_info.username, node_info.ip_addr_string(), command);

    execute_local_command(&ssh_command)
}

pub fn scp_copy_of_local_source_path_to_remote_destination_path(source_path: &str, destination_path: &str, node_info: &NodeInfo) -> Child {
    let scp_command = format!("scp -i {} -r {} {}@{}:distributed_swmr_registers_remote_directory/{}", 
        node_info.key_path, 
        source_path, 
        node_info.username, 
        node_info.ip_addr_string(), 
        destination_path);

    execute_local_command(&scp_command)
}

pub fn execute_local_command(command: &str) -> Child {
    Command::new("/bin/bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect(&format!("Failed to execute the command: {}", command))
}
