
use std::net::SocketAddr;
use std::net::IpAddr::V4;
use std::net::ToSocketAddrs;
use std::fs;
use std::collections::HashSet;
use std::time::Duration;

use clap::{Arg, App, ArgMatches, SubCommand};

use crate::run_scenario::Scenario;


lazy_static! {
    pub static ref ARGUMENTS: Arguments = Arguments::new();
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


#[derive(Debug)]
pub struct Arguments {
    pub node_infos: HashSet<NodeInfo>,
    pub optimize_string: String,
    pub print_client_operations_string: String,
    pub run_length_string: String,
    pub install: bool
}

impl Arguments {
    fn new() -> Arguments {
        let matches = get_matches();

        let node_infos = node_infos_from_matches(&matches);
        let scenarios = scenarios_from_matches(&matches);
        let optimize_string = optimize_string_from_matches(&matches);
        let print_client_operations_string = print_client_operations_string_from_matches(&matches);
        let run_length_string = run_length_string_from_matches(&matches);
        let install = install_from_matches(&matches);

        Arguments {
            node_infos: node_infos,
            optimize_string: optimize_string,
            print_client_operations_string: print_client_operations_string,
            run_length_string: run_length_string,
            install: install
        }
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


fn node_infos_from_matches(matches: &ArgMatches<'static>) -> HashSet<NodeInfo> {
    let hosts_file_path = matches.value_of("hosts-file").unwrap();
    let string = fs::read_to_string(hosts_file_path).expect("Unable to read file");
    node_infos_from_string(string)
}

fn node_infos_from_string(string: String) -> HashSet<NodeInfo> {
    let mut node_infos = HashSet::new();

    for line in string.lines() {
        let components: Vec<&str> = line.split(",").collect();
        let node_id = components[0].parse().unwrap();
        let socket_addr = components[1].to_socket_addrs().unwrap().next().unwrap();
        let key_path = components[2].to_string();
        let username = components[3].to_string();

        let node_info = NodeInfo {
            node_id: node_id,
            socket_addr: socket_addr,
            key_path: key_path,
            username: username
        };

        node_infos.insert(node_info);
    }

    node_infos
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

fn run_length_string_from_matches(matches: &ArgMatches<'static>) -> String {
    matches.value_of("run-length").unwrap().to_string()
}

fn install_from_matches(matches: &ArgMatches<'static>) -> bool {
    matches.is_present("install")
}