
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

    let my_run_scenario = RunScenario::new(3, 3, 3);

    let hosts_file = "hosts.txt";
    let optimize_string = "";

    let command = format!("cargo run --manifest-path ../remote_starter/Cargo.toml -- {} -r {} -w {} -e {} -l 10", 
        hosts_file, 
        my_run_scenario.number_of_readers, 
        my_run_scenario.number_of_writers,
        optimize_string);

    execution::execute_local_command(&command);



}
