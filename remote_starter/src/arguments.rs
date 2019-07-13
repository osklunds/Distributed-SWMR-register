
use std::net::SocketAddr;
use std::net::IpAddr::V4;
use std::net::ToSocketAddrs;
use std::fs;
use std::collections::HashSet;
use std::time::Duration;

use clap::{Arg, App, ArgMatches};


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
    node_infos: HashSet<NodeInfo>,
    number_of_writers: i32,
    number_of_readers: i32,
    release_mode_string: String,
    print_client_operations_string: String,
    run_length: String,
    record_evaluation_info_string: String
}

impl Arguments {
    fn new() -> Arguments {
        let matches = get_matches();

        let node_infos = node_infos_from_matches(&matches);
        let number_of_writers = number_of_writers_from_matches(&matches);
        let number_of_readers = number_of_readers_from_matches(&matches);
        let release_mode_string = release_mode_string_from_matches(&matches);
        let print_client_operations_string = print_client_operations_string_from_matches(&matches);
        let run_length_string = run_length_string_from_matches(&matches);
        let record_evaluation_info_string = record_evaluation_info_string_from_matches(&matches);

        Arguments {
            node_infos: node_infos,
            number_of_writers: number_of_writers,
            number_of_readers: number_of_readers,
            release_mode_string: release_mode_string,
            print_client_operations_string: print_client_operations_string,
            run_length_string: run_length_string,
            record_evaluation_info_string: record_evaluation_info_string
        }
    }
}

fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Remote starter")
        .version("0.1")
        .author("Oskar Lundström")
        .about("Todo")

        .arg(Arg::with_name("hosts-file")
            .required(true)
            .takes_value(true)
            .help("The file with host ids, addresses and ports."))

        .arg(Arg::with_name("number-of-writers")
            .required(true)
            .takes_value(true)
            .short("w")
            .long("number-of-writers")
            .help("The number of nodes that should write."))

        .arg(Arg::with_name("number-of-readers")
            .required(true)
            .takes_value(true)
            .short("r")
            .long("number-of-readers")
            .help("The number of nodes that should read."))

        .arg(Arg::with_name("run-length")
            .takes_value(true)
            .required(true)
            .short("l")
            .long("run-length")
            .help("The number of seconds the program should run for. If 0 is given, the program will until aborted with Ctrl-C."))

        .arg(Arg::with_name("record-evaluation-info")
            .short("e")
            .long("record-evaluation-info")
            .takes_value(false)
            .help("Record information used for the evaluation, such as latency and number of messages sent. If not included, the performance might be slightly higher."))

        .arg(Arg::with_name("optimize")
            .takes_value(false)
            .short("o")
            .long("optimize")
            .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance."))

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
        let id = components[0].parse().unwrap();
        let socket_addr = components[1].to_socket_addrs().unwrap().next().unwrap();
        let key_path = components[2].to_string();
        let username = components[3].to_string();

        let node_info = NodeInfo {
            node_id: id,
            socket_addr: socket_addr,
            key_path: key_path,
            username: username
        };

        node_infos.insert(node_info);
    }

    node_infos
}

fn number_of_writers_from_matches(matches: &ArgMatches<'static>) -> i32 {
    if let Some(number_of_writers) = matches.value_of("number-of-writers") {
        number_of_writers.parse().unwrap()
    } else {
        0
    }
}

fn number_of_readers_from_matches(matches: &ArgMatches<'static>) -> i32 {
    if let Some(number_of_readers) = matches.value_of("number-of-readers") {
        number_of_readers.parse().unwrap()
    } else {
        0
    }
}

fn release_mode_string_from_matches(matches: &ArgMatches<'static>) -> String {
    if matches.is_present("optimize") {
        "--release".to_string()
    } else {
        "".to_string()
    }
}

fn print_client_operations_string_from_matches(matches: &ArgMatches<'static>) -> String {
    if matches.is_present("print-client-operations") {
        "--print-client-operations".to_string()
    } else {
        "".to_string()
    }
}

fn run_length_string_from_matches(matches: &ArgMatches<'static>) -> String {
    matches.value_of("run-length").unwrap()
}

fn record_evaluation_info_string_from_matches(matches: &ArgMatches<'static>) -> String {
    if matches.is_present("record-evaluation-info") {
        "--record-evaluation-info".to_string()
    } else {
        "".to_string()
    }
}