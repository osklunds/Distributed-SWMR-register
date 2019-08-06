
use std::net::SocketAddr;
use std::net::IpAddr::V4;
use std::net::ToSocketAddrs;
use std::fs;
use std::collections::{HashSet, HashMap};
use std::time::Duration;

use clap::{Arg, App, ArgMatches, SubCommand};

use crate::run_scenario::Scenario;
use crate::run_result::{RunResult, ScenarioResults, ScenarioResult};


lazy_static! {
    pub static ref ARGUMENTS: Arguments = Arguments::new();
}


pub enum Arguments {
    Install(InstallArguments),
    Gather(GatherArguments),
    Aggregate(AggregateArguments)
}

impl Arguments {
    fn new() -> Arguments {
        let matches = get_matches();

        if let Some(install_matches) = matches.subcommand_matches("install") {
            Arguments::Install(InstallArguments::from_matches(&install_matches))
        } else if let Some(gather_matches) = matches.subcommand_matches("gather") {
            Arguments::Gather(GatherArguments::from_matches(&gather_matches))
        } else if let Some(aggregate_matches) = matches.subcommand_matches("aggregate") {
            Arguments::Aggregate(AggregateArguments::from_matches(&aggregate_matches))
        } else {
            panic!("No correct subcommand was provided.")
        }
    }
}


pub struct InstallArguments {
    pub hosts_file: String,
    pub optimize_string: String,
}

impl InstallArguments {
    fn from_matches(matches: &ArgMatches<'static>) -> InstallArguments {
        let hosts_file = hosts_file_from_matches(matches);
        let optimize_string = optimize_string_from_matches(matches);

        InstallArguments {
            hosts_file: hosts_file,
            optimize_string: optimize_string
        }
    }
}


pub struct GatherArguments {
    pub hosts_file: String,
    pub scenarios: HashSet<Scenario>,
    pub result_file_path: String,
    pub optimize_string: String,
    pub print_client_operations_string: String,
}

impl GatherArguments {
    fn from_matches(matches: &ArgMatches<'static>) -> GatherArguments {
        let hosts_file = hosts_file_from_matches(matches);
        let scenarios = scenarios_from_matches(matches);
        let result_file_path = result_file_path_from_matches(matches);
        let optimize_string = optimize_string_from_matches(matches);
        let print_client_operations_string = print_client_operations_string_from_matches(matches);

        GatherArguments {
            hosts_file: hosts_file,
            scenarios: scenarios,
            result_file_path: result_file_path,
            optimize_string: optimize_string,
            print_client_operations_string: print_client_operations_string
        }
    }
}


pub struct AggregateArguments {
    pub run_results: HashMap<Scenario, Vec<ScenarioResult>>
}

impl AggregateArguments {
    fn from_matches(matches: &ArgMatches<'static>) -> AggregateArguments {
        let run_results = run_results_from_matches(matches);

        AggregateArguments {
            run_results: run_results
        }
    }
}


pub type NodeId = i32;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NodeInfo {
    pub node_id: NodeId,
    pub socket_addr: SocketAddr,
    pub key_path: String,
    pub username: String
}

impl NodeInfo {
    pub fn ip_addr_string(&self) -> String {
        format!("{}", self.socket_addr.ip())
    }
}



fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Evaluator")
        .version("0.1")
        .author("Oskar Lundström")
        .about("A helper utility that runs the application on, and gathers and aggregates evaluation info from, remote machines.")

        .subcommand(SubCommand::with_name("install")
            .about("Will install Rust and the source code on the hosts.")
            
            .arg(Arg::with_name("hosts-file")
                .required(true)
                .takes_value(true)
                .help("The file with node ids, addresses, ports, ssh key paths and usernames."))

            .arg(Arg::with_name("optimize")
                .takes_value(false)
                .short("o")
                .long("optimize")
                .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance.")))

        .subcommand(SubCommand::with_name("gather")
            .about("Will run each scenario ones and gather the results in a file. The results-file will be built upon, and if a scenario already exists there, it will not be run again.")
            
            .arg(Arg::with_name("hosts-file")
                .required(true)
                .takes_value(true)
                .help("The file with node ids, addresses, ports, ssh key paths and usernames."))

            .arg(Arg::with_name("scenario-file")
                .required(true)
                .takes_value(true)
                .help("The file with scenarios to run."))

            .arg(Arg::with_name("result-file")
                .required(true)
                .takes_value(true)
                .help("The file to which the results are stored."))

            .arg(Arg::with_name("optimize")
                .takes_value(false)
                .short("o")
                .long("optimize")
                .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance.")))

            .arg(Arg::with_name("print-client-operations")
            .short("p")
            .long("print-client-operations")
            .takes_value(false)
            .help("Print when a read/write operation starts/ends. If not included, the performance might be slightly higher."))

        .subcommand(SubCommand::with_name("aggregate")
            .about("Will aggregate multiple result-files to generate aggregated results, according to what you have programatically defined.")

            .arg(Arg::with_name("result-files")
                .required(true)
                .takes_value(true)
                .help("The files with results. Each file should have the same scenarios as the other files.")))

        .get_matches()
}


fn hosts_file_from_matches(matches: &ArgMatches<'static>) -> String {
    matches.value_of("hosts-file").unwrap().to_string()
}

fn scenarios_from_matches(matches: &ArgMatches<'static>) -> HashSet<Scenario> {
    let scenarios_file_path = matches.value_of("scenario-file").unwrap();
    let string = fs::read_to_string(scenarios_file_path).expect("Unable to read the scenarios file.");
    scenarios_from_string(string)
}

fn scenarios_from_string(string: String) -> HashSet<Scenario> {
    let mut scenarios = HashSet::new();

    for line in string.lines() {
        let components: Vec<&str> = line.split(",").collect();
        let number_of_nodes = components[0].parse().unwrap();
        let number_of_readers = components[1].parse().unwrap();
        let number_of_writers = components[2].parse().unwrap();

        let scenario = Scenario::new(number_of_nodes, number_of_readers, number_of_writers);

        scenarios.insert(scenario);
    }

    scenarios
}

fn result_file_path_from_matches(matches: &ArgMatches<'static>) -> String {
    matches.value_of("result-file").unwrap().to_string()
}

fn optimize_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("optimize") {
        true  => "--release".to_string(),
        false => "".to_string()
    }
}

fn print_client_operations_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("print-client-operations") {
        true  => "--print-client-operations".to_string(),
        false => "".to_string()
    }
}

fn run_results_from_matches(matches: &ArgMatches<'static>) -> HashMap<Scenario, Vec<ScenarioResult>> {
    let result_strings = matches.values_of("result-file").unwrap().map(|result_file| fs::read_to_string(result_file).unwrap());
    let mut aggregated_scenario_results: HashMap<Scenario, Vec<ScenarioResult>> = HashMap::new();

    for result_string in result_strings {
        let scenario_results: HashMap<Scenario, ScenarioResult> = serde_json::from_str(&result_string).expect("Could not deserialize");

        for (scenario, scenario_result) in scenario_results.iter() {
            let exisiting_results_for_scenario: Option<&Vec<ScenarioResult>> = aggregated_scenario_results.get(scenario);

            if let Some(exisiting_results_for_scenario) = exisiting_results_for_scenario {
                let mut exisiting_results_for_scenario = exisiting_results_for_scenario.to_vec();
                exisiting_results_for_scenario.push(scenario_result.to_vec());

                aggregated_scenario_results.insert(*scenario, exisiting_results_for_scenario);
            } else {
                let lone_result = vec![scenario_result.to_vec()];

                aggregated_scenario_results.insert(*scenario, lone_result);
            }
        }
    }

    aggregated_scenario_results
}
