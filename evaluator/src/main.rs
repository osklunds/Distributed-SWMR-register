
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

use std::path::Path;
use std::fs;
use std::collections::HashMap;

mod run_result;
mod run_scenario;
mod arguments;
mod execution;

use run_scenario::*;
use run_result::{RunResult, NodeId};
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
        let result_file_path = &arguments.result_file_path;

        create_result_file_if_not_existing(&arguments.result_file_path);
        let mut results = read_result_file(&arguments.result_file_path);

        for scenario in &arguments.scenarios {
            if !results.contains_key(&scenario) {
                loop {
                    let command = format!("cargo run --manifest-path ../remote_starter/Cargo.toml -- {} -r {} -w {} -e {} -l 3 {}",
                        hosts_file,
                        scenario.number_of_readers,
                        scenario.number_of_writers,
                        optimize_string,
                        print_client_operations_string);
                    execution::execute_local_command(&command).wait().expect("Could not wait for the gather command for remote_starter.");

                    let mut results_for_this_scenario = HashMap::new();
                    let mut soundness_violator = None;

                    for node_info in &arguments.node_infos {
                        let file_name = format!("node{:0>3}.eval", node_info.node_id);
                        execution::execute_scp_download_of_path(&file_name, &node_info).wait().unwrap();
                        
                        let json = fs::read_to_string(&file_name).expect("Could not read a run result.");
                        let run_result: RunResult = serde_json::from_str(&json).expect("Could not parse a run result.");

                        if !run_result.is_valid(scenario.number_of_nodes) {
                            soundness_violator = Some(node_info.node_id);
                        }

                        results_for_this_scenario.insert(node_info.node_id, run_result);
                    }

                    if let Some(soundness_violator) = soundness_violator {
                        println!("The result for {:?} is not sound, violated by {}, so it is re-run.", scenario, soundness_violator);
                    } else {
                        results.insert(*scenario, results_for_this_scenario);
                        break;
                    }
                }
            }

            let json = serde_json::to_string(&results).expect("Could not serialize the result.");
            fs::write(result_file_path, &json).expect("Could not write the result file.");
        }
    }
}



fn create_result_file_if_not_existing(result_file_path: &Path) {
    if result_file_path.is_dir() {
        fs::remove_dir(result_file_path).expect("Could not remove a result file directory.");
    }
    if !result_file_path.is_file() {
        let empty_result: HashMap<Scenario, HashMap<NodeId, RunResult>> = HashMap::new();
        let json = serde_json::to_string(&empty_result).expect("Could not serialize the empty result set.");
        fs::write(result_file_path, json).expect("Could not write the empty result file.");
    }
}

fn read_result_file(result_file_path: &Path) -> HashMap<Scenario, HashMap<NodeId, RunResult>> {
    let json = fs::read_to_string(result_file_path).expect("Could not read the result file.");
    serde_json::from_str(&json).expect("Could not parse the result file.")
}


