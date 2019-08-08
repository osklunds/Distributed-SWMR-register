
use std::collections::HashSet;
use std::fs;
use std::net::ToSocketAddrs;

use clap::{Arg, App, ArgMatches, AppSettings};

use crate::node_info::NodeInfo;


pub fn hosts_file(help_text: &'static str) -> Arg<'static, 'static> {
    Arg::with_name("hosts-file")
        .required(true)
        .takes_value(true)
        .help(help_text)
}

pub fn hosts_file_from_matches(matches: &ArgMatches<'static>) -> String {
    matches.value_of("hosts-file").unwrap().to_string()
}

pub fn node_infos_from_matches(matches: &ArgMatches<'static>) -> HashSet<NodeInfo> {
    let hosts_file_path = matches.value_of("hosts-file").unwrap();
    let string = fs::read_to_string(hosts_file_path).expect("Unable to read file");
    node_infos_from_string(string)
}

pub fn node_infos_from_string(string: String) -> HashSet<NodeInfo> {
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

pub fn number_of_writers() -> Arg<'static, 'static> {
    Arg::with_name("number-of-writers")
        .required(false)
        .takes_value(true)
        .default_value("0")
        .short("w")
        .long("number-of-writers")
        .help("The number of nodes that should write.")
}

pub fn number_of_writers_from_matches(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-writers").unwrap().parse().unwrap()
}

pub fn number_of_readers() -> Arg<'static, 'static> {
    Arg::with_name("number-of-readers")
        .required(false)
        .takes_value(true)
        .default_value("0")
        .short("r")
        .long("number-of-readers")
        .help("The number of nodes that should read.")
}

pub fn number_of_readers_from_matches(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-readers").unwrap().parse().unwrap()
}

pub fn run_length() -> Arg<'static, 'static> {
    Arg::with_name("run-length")
        .required(false)
        .takes_value(true)
        .default_value("0")
        .short("l")
        .long("run-length")
        .help("The number of seconds the program should run for. If 0 is given, the program will until aborted with Ctrl-C.")
}

pub fn run_length_string_from_matches(matches: &ArgMatches<'static>) -> String {
    matches.value_of("run-length").unwrap().to_string()
}

pub fn record_evaluation_info() -> Arg<'static, 'static> {
    Arg::with_name("record-evaluation-info")
        .short("e")
        .long("record-evaluation-info")
        .takes_value(false)
        .help("Record information used for the evaluation, such as latency and number of messages sent. If not done, the performance might be slightly higher.")
}

pub fn record_evaluation_info_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("record-evaluation-info") {
        true  => "--record-evaluation-info".to_string(),
        false => "".to_string()
    }
}

pub fn optimize() -> Arg<'static, 'static> {
    Arg::with_name("optimize")
        .takes_value(false)
        .short("o")
        .long("optimize")
        .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance.")
}

pub fn release_mode_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("optimize") {
        true  => "--release".to_string(),
        false => "".to_string()
    }
}

pub fn print_client_operations() -> Arg<'static, 'static> {
    Arg::with_name("print-client-operations")
        .short("p")
        .long("print-client-operations")
        .takes_value(false)
        .help("Print when a read/write operation starts/ends. If not included, the performance might be slightly higher.")
}

pub fn print_client_operations_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("print-client-operations") {
        true  => "--print-client-operations".to_string(),
        false => "".to_string()
    }
}