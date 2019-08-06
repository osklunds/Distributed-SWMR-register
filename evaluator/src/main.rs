
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

mod run_result;
mod run_scenario;
mod arguments;
mod execution;

use run_scenario::*;
use run_result::RunResult;
use arguments::{Arguments, ARGUMENTS};

fn main() {
    let arguments: &Arguments = &ARGUMENTS;

    if let Arguments::Install(arguments) = arguments {
        let hosts_file = &arguments.hosts_file;
        let optimize_string = &arguments.optimize_string;
        let command = format!("cargo run --manifest-path ../remote_starter/Cargo.toml -- {} -i {}", hosts_file, optimize_string);

        execution::execute_local_command(&command).wait().expect("Error waiting for the execution of the install command.");
    } else if let Arguments::Gather(arguments) = arguments {
        let hosts_file = &arguments.hosts_file;
        let optimize_string = &arguments.optimize_string;
        let print_client_operations_string = &arguments.print_client_operations_string;

        create_result_file_if_not_existing(&arguments.result_file_path);


    }

    

    /*
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

    */


}

fn create_result_file_if_not_existing(result_file_path: &str) {
    let result_file_path = PathBuf::from(result_file_path);
    if result_file_path.is_dir() {
        fs::remove_dir(&result_file_path).expect("Could not remove a result file directory.");
    }
    if !result_file_path.is_file() {
        let empty_result: HashMap<Scenario, RunResult> = HashMap::new();
        let json = serde_json::to_string(&empty_result).expect("Could not serialize the empty result set.");
        fs::write(&result_file_path, json).expect("Could not write the empty result file.");

    }
}
