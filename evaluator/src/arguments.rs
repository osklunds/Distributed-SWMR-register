
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::fs;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap};
use std::time::Duration;

use clap::{Arg, App, ArgMatches, SubCommand, AppSettings};

use commons::node_info::{NodeInfo, NodeId};
use commons::run_result::RunResult;
use commons::arguments;

use crate::scenario::Scenario;


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
        let hosts_file = arguments::hosts_file_from_matches(matches);
        let optimize_string = optimize_string_from_matches(matches);

        InstallArguments {
            hosts_file: hosts_file,
            optimize_string: optimize_string
        }
    }
}


pub struct GatherArguments {
    pub hosts_file: String,
    pub node_infos: HashSet<NodeInfo>,
    pub scenarios: HashSet<Scenario>,
    pub result_file_path: PathBuf,
    pub optimize_string: String,
    pub print_client_operations_string: String,
    pub run_length_string: String
}

impl GatherArguments {
    fn from_matches(matches: &ArgMatches<'static>) -> GatherArguments {
        GatherArguments {
            hosts_file: arguments::hosts_file_from_matches(matches),
            node_infos: arguments::node_infos_from_matches(matches),
            scenarios: scenarios_from_matches(matches),
            result_file_path: result_file_path_from_matches(matches),
            optimize_string: optimize_string_from_matches(matches),
            print_client_operations_string: arguments::print_client_operations_string_from_matches(matches),
            run_length_string: arguments::run_length_string_from_matches(&matches)
        }
    }
}


pub struct AggregateArguments {
    pub run_results: HashMap<Scenario, Vec<HashMap<NodeId, RunResult>>>
}

impl AggregateArguments {
    fn from_matches(matches: &ArgMatches<'static>) -> AggregateArguments {
        let run_results = run_results_from_matches(matches);

        AggregateArguments {
            run_results: run_results
        }
    }
}


fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Evaluator")
        .about("A helper utilty that gathers evaluation results and aggregates them")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)

        .subcommand(SubCommand::with_name("install")
            .about("Will install Rust and the source code on the (remote) hosts.")
            
            .arg(arguments::hosts_file())
            .arg(arguments::optimize()))

        .subcommand(SubCommand::with_name("gather")
            .about("Will run each scenario ones and gather the results in a file. The results-file will be built upon, and if a scenario already exists there, it will not be run again.")
            
            .arg(arguments::hosts_file())
            .arg(scenario_file_argument())
            .arg(result_file_argument())
            .arg(arguments::optimize())
            .arg(run_length_argument())
            .arg(arguments::print_client_operations()))

        .subcommand(SubCommand::with_name("aggregate")
            .about("Will aggregate multiple result-files to generate aggregated results, according to what you have programatically defined.")

            .arg(result_files_argument()))

        .get_matches()
}


fn scenario_file_argument() -> Arg<'static, 'static> {
    Arg::with_name("scenario-file")
        .required(true)
        .takes_value(true)
        .help("The file with scenarios to run.")
}

fn result_file_argument() -> Arg<'static, 'static> {
    Arg::with_name("result-file")
        .required(true)
        .takes_value(true)
        .help("The file in which the results are stored.")
}

fn run_length_argument() -> Arg<'static, 'static> {
    Arg::with_name("run-length")
        .required(false)
        .takes_value(true)
        .default_value("3")
        .short("l")
        .long("run-length")
        .help("The number of seconds the program should run for. If 0 is given, the program will run forever. Avoid this value.")
}

fn result_files_argument() -> Arg<'static, 'static> {
    Arg::with_name("result-files")
        .required(true)
        .takes_value(true)
        .help("The files with results. Each file should have the same scenarios as the other files.")
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

fn result_file_path_from_matches(matches: &ArgMatches<'static>) -> PathBuf {
    let as_str = matches.value_of("result-file").unwrap();
    PathBuf::from(as_str)
}

fn optimize_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("optimize") {
        true  => "--optimize".to_string(),
        false => "".to_string()
    }
}

fn run_results_from_matches(matches: &ArgMatches<'static>) -> HashMap<Scenario, Vec<HashMap<NodeId, RunResult>>> {
    HashMap::new()
    /*
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
    */
}
