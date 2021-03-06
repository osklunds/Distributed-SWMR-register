use std::collections::HashSet;
use std::fs;
use std::net::ToSocketAddrs;

use clap::{Arg, ArgMatches};
use colored::Color;
use colored::Color::*;

use crate::node_info::NodeInfo;
use crate::types::{Int, NodeId};

pub fn hosts_file(help_text: &'static str) -> Arg<'static, 'static> {
    Arg::with_name("hosts-file")
        .required(true)
        .takes_value(true)
        .help(help_text)
}

pub fn hosts_file_from_matches(matches: &ArgMatches<'static>) -> String {
    matches
        .value_of("hosts-file")
        .expect("hosts-file not found in matches.")
        .to_string()
}

pub fn node_infos_from_matches(
    matches: &ArgMatches<'static>,
) -> HashSet<NodeInfo> {
    let hosts_file_path = hosts_file_from_matches(matches);
    let string = fs::read_to_string(hosts_file_path)
        .expect("Unable to read the hosts file.");
    node_infos_from_string(string)
}

pub fn node_infos_from_string(string: String) -> HashSet<NodeInfo> {
    let mut node_infos = HashSet::new();

    for line in string.lines() {
        let components: Vec<&str> = line.split(",").collect();
        let node_id =
            components[0].parse().expect("Could not parse node id.");
        let socket_addr = components[1]
            .to_socket_addrs()
            .expect("Could not transform to socket addrs.")
            .next()
            .expect("No socket addrs provided.");
        let key_path = components[2].to_string();
        let username = components[3].to_string();
        let script_path = components[4].to_string();

        let node_info = NodeInfo {
            node_id: node_id,
            socket_addr: socket_addr,
            key_path: key_path,
            username: username,
            script_path: script_path,
        };

        node_infos.insert(node_info);
    }

    node_infos
}

pub fn should_write() -> Arg<'static, 'static> {
    Arg::with_name("write")
        .required(false)
        .takes_value(false)
        .short("w")
        .long("write")
        .help("If the writer node should write.")
}

pub fn should_write_from_matches(
    matches: &ArgMatches<'static>,
) -> bool {
    matches.is_present("write")
}

pub fn number_of_readers() -> Arg<'static, 'static> {
    Arg::with_name("number-of-readers")
        .required(false)
        .takes_value(true)
        .default_value("0")
        .short("r")
        .long("number-of-readers")
        .help("The number of nodes that should read. If the writer node is instructed to write, the number of readers must be at most one less than the total number of nodes.")
}

pub fn number_of_readers_from_matches(
    matches: &ArgMatches<'static>,
) -> Int {
    matches
        .value_of("number-of-readers")
        .expect("Number of readers arg not existing.")
        .parse()
        .expect("Could not parse number of readers.")
}

pub fn run_length() -> Arg<'static, 'static> {
    Arg::with_name("run-length")
        .required(false)
        .takes_value(true)
        .default_value("0")
        .short("l")
        .long("run-length")
        .help("The number of seconds the program should run for. If 0 is given, the program will run until aborted with Ctrl-C.")
}

pub fn run_length_string_from_matches(
    matches: &ArgMatches<'static>,
) -> String {
    matches
        .value_of("run-length")
        .expect("run length arg not existing.")
        .to_string()
}

pub fn optimize() -> Arg<'static, 'static> {
    Arg::with_name("optimize")
        .takes_value(false)
        .short("o")
        .long("optimize")
        .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance.")
}

pub fn release_mode_string_from_matches(
    matches: &ArgMatches<'static>,
) -> String {
    match matches.is_present("optimize") {
        true => "--release".to_string(),
        false => "".to_string(),
    }
}

pub fn print_client_operations() -> Arg<'static, 'static> {
    Arg::with_name("print-client-operations")
        .takes_value(false)
        .short("p")
        .long("print-client-operations")
        .help("Print when a read/write operation starts/ends. If not included, the performance might be slightly higher.")
}

pub fn print_client_operations_string_from_matches(
    matches: &ArgMatches<'static>,
) -> String {
    match matches.is_present("print-client-operations") {
        true => "--print-client-operations".to_string(),
        false => "".to_string(),
    }
}

pub fn color_from_node_id(node_id: NodeId) -> Color {
    let colors = vec![Black, Red, Green, Yellow, Blue, Magenta, Cyan];
    colors[(node_id as usize) % colors.len()]
}

pub fn run_result_file_name_from_node_id(node_id: NodeId) -> String {
    format!("node{:0>6}.eval", node_id)
}
