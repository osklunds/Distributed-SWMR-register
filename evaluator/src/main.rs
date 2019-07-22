
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

mod run_result;
mod run_scenario;
mod arguments;
mod execution;

use run_scenario::*;
use arguments::ARGUMENTS;

fn main() {
    &ARGUMENTS.node_infos;

    let my_scenario = Scenario::new(3, 3, 3);

    let hosts_file = "hosts.txt";
    let optimize_string = "";

    let command = format!("cargo run --manifest-path ../remote_starter/Cargo.toml -- {} -r {} -w {} -e {} -l 3", 
        hosts_file, 
        my_scenario.number_of_readers, 
        my_scenario.number_of_writers,
        optimize_string);

    execution::execute_local_command(&command).wait().unwrap();

    for node_info in &ARGUMENTS.node_infos {
        let file_name = format!("node{:0>3}.eval", node_info.node_id);
        execution::execute_scp_download_of_path(&file_name, &node_info).wait().unwrap();
    }



}
