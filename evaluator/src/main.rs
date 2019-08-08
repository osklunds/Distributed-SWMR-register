
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[macro_use]
extern crate lazy_static;
extern crate serde;

use std::path::Path;
use std::fs;
use std::collections::HashMap;

use commons::execution;
use commons::node_info::NodeInfo;
use commons::run_result::RunResult;

mod run_result;
mod scenario;
mod arguments;

use scenario::*;
use arguments::*;


fn main() {
    let arguments: &Arguments = &ARGUMENTS;

    if let Arguments::Install(arguments) = arguments {
        run_install_subcommand(arguments);
    } else if let Arguments::Gather(arguments) = arguments {
        run_gather_subcommand(arguments);
    }
}

fn run_install_subcommand(arguments: &InstallArguments) {
    let hosts_file = &arguments.hosts_file;
    let optimize_string = &arguments.optimize_string;
    let command = format!("cargo run --manifest-path ../remote_starter/Cargo.toml -- {} -i {}", hosts_file, optimize_string);

    execution::execute_local_command(&command).wait().expect("Error waiting for the execution of the install command.");
}

fn run_gather_subcommand(arguments: &GatherArguments) {
    let result_file_path = &arguments.result_file_path;
    create_result_file_if_not_existing(result_file_path);
    let mut results = read_result_file(result_file_path);

    for scenario in &arguments.scenarios {
        run_scenario_if_not_already_run_and_insert_result(&scenario, arguments, &mut results);
        save_results_to_file(&results, result_file_path);
    }
}

fn create_result_file_if_not_existing(result_file_path: &Path) {
    if result_file_path.is_dir() {
        fs::remove_dir_all(result_file_path).expect("Could not remove a result file directory.");
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

fn run_scenario_if_not_already_run_and_insert_result(scenario: &Scenario, arguments: &GatherArguments, results: &mut HashMap<Scenario, HashMap<NodeId, RunResult>>) {
    if !results.contains_key(&scenario) {
        let result = run_scenario(scenario, arguments);
        results.insert(*scenario, result);
    }
}

fn run_scenario(scenario: &Scenario, arguments: &GatherArguments) -> HashMap<NodeId, RunResult> {
    loop {
        match run_scenario_once(scenario, arguments) {
            Some(result) => return result,
            None => {}
        }
    }
}

fn run_scenario_once(scenario: &Scenario, arguments: &GatherArguments) -> Option<HashMap<NodeId, RunResult>> {
    execute_command_for_scenario_and_arguments(scenario, arguments);

    let results_for_this_scenario = collect_results_from_scenario_and_arguments(scenario, arguments);

    match results_for_this_scenario {
        CollectResult::Success(results_for_this_scenario) => return Some(results_for_this_scenario),
        CollectResult::Failure(soundness_violator) => {
            println!("The result for {:?} is not sound, violated by {}.", scenario, soundness_violator);
            return None
        }
    }
}

fn execute_command_for_scenario_and_arguments(scenario: &Scenario, arguments: &GatherArguments) {
    let command = format!("cargo run --manifest-path ../remote_starter/Cargo.toml -- {} -r {} -w {} -e {} -l {} {}",
            arguments.hosts_file,
            scenario.number_of_readers,
            scenario.number_of_writers,
            arguments.optimize_string,
            arguments.run_length_string,
            arguments.print_client_operations_string);
    execution::execute_local_command(&command).wait().expect("Could not wait for the gather command for remote_starter.");
}

fn collect_results_from_scenario_and_arguments(scenario: &Scenario, arguments: &GatherArguments) -> CollectResult {
    let mut results_for_this_scenario = HashMap::new();

    for node_info in &arguments.node_infos {
        let run_result = collect_result_for_node_info(&node_info);

        if run_result.is_valid(scenario.number_of_nodes) {
            results_for_this_scenario.insert(node_info.node_id, run_result);
        } else {
            return CollectResult::Failure(node_info.node_id);
        }
    }

    return CollectResult::Success(results_for_this_scenario);
}

enum CollectResult {
    Success(HashMap<NodeId, RunResult>),
    Failure(NodeId)
}

fn collect_result_for_node_info(node_info: &NodeInfo) -> RunResult {
    let file_name = format!("node{:0>3}.eval", node_info.node_id);
    execution::scp_copy_of_remote_source_path_to_local_destination_path(&file_name, &file_name, &node_info).wait().unwrap();
    
    let json = fs::read_to_string(&file_name).expect("Could not read a run result.");
    serde_json::from_str(&json).expect("Could not parse a run result.")
}

fn save_results_to_file(results: &HashMap<Scenario, HashMap<NodeId, RunResult>>, result_file_path: &Path) {
    let json = serde_json::to_string(&results).expect("Could not serialize the result.");
    fs::write(result_file_path, &json).expect("Could not write the result file.");
}