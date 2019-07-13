
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::net::SocketAddr;
use std::fs;
use std::time::Duration;


use colored::*;
use clap::{Arg, App, ArgMatches};


pub type NodeId = i32;


lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}


#[derive(Debug)]
pub struct Settings {
    node_id: NodeId,
    socket_addrs: HashMap<NodeId, SocketAddr>,
    terminal_color: Color,
    should_write: bool,
    should_read: bool,
    print_client_operations: bool,
    run_length: Duration,
    record_evaluation_info: bool,
}

impl Settings {
    fn new() -> Settings {
        let matches = get_matches();

        let node_id = node_id_from_matches(&matches);
        let socket_addrs = socket_addrs_from_matches(&matches);
        let color = color_from_matches(&matches);
        let run_length = run_length_from_matches(&matches);

        Settings {
            node_id: node_id,
            socket_addrs: socket_addrs,
            terminal_color: color,
            should_write: matches.is_present("write"),
            should_read: matches.is_present("read"),
            print_client_operations: matches.is_present("print_client_operations"),
            run_length: run_length,
            record_evaluation_info: matches.is_present("evaluation-length"),
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn socket_addrs(&self) -> &HashMap<NodeId, SocketAddr> {
        &self.socket_addrs
    }

    pub fn terminal_color(&self) -> Color {
        self.terminal_color
    }

    #[allow(dead_code)]
    pub fn record_evaluation_info(&self) -> bool {
        self.record_evaluation_info
    }

    pub fn print_client_operations(&self) -> bool {
        self.print_client_operations
    }

    pub fn number_of_nodes(&self) -> usize {
        self.socket_addrs.len()
    }

    pub fn node_ids(&self) -> HashSet<NodeId> {
        HashSet::from_iter(self.socket_addrs.keys().map(|id| *id))
    }

    pub fn should_read(&self) -> bool {
        self.should_read
    }

    pub fn should_write(&self) -> bool {
        self.should_write
    }

    pub fn run_length(&self) -> Duration {
        self.run_length
    }
}

fn get_matches() -> ArgMatches<'static> {
    let colors = &["Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan"];
    App::new("Distributed SWMR registers")
        .version("0.1")
        .author("Oskar Lundström")
        .about("Todo")

        .arg(Arg::with_name("node-id")
            .required(true)
            .takes_value(true)
            .help("The integer id of this node instance."))

        .arg(Arg::with_name("hosts-file")
            .required(true)
            .takes_value(true)
            .help("The file with host ids, addresses and ports."))

        .arg(Arg::with_name("write")
            .short("w")
            .long("write")
            .takes_value(false)
            .help("Makes this node perform write operations."))

        .arg(Arg::with_name("read")
            .short("r")
            .long("read")
            .takes_value(false)
            .help("Makes this node perform read operations."))

        .arg(Arg::with_name("print_client_operations")
            .short("p")
            .long("print-client-operations")
            .takes_value(false)
            .help("Print when a read/write operation starts/ends. If not included, the performance might be slightly higher."))

        .arg(Arg::with_name("run-length")
            .takes_value(true)
            .required(true)
            .help("The number of seconds the program should run for."))

        .arg(Arg::with_name("record-evaluation-info")
            .short("e")
            .long("record-evaluation-info")
            .takes_value(false)
            .help("Record information used for the evaluation, such as latency and number of messages sent. If not included, the performance might be slightly higher."))

        .arg(Arg::with_name("color")
            .takes_value(true)
            .possible_values(colors)
            .default_value("Black")
            .help("The color of the terminal output"))
        
        .get_matches()
}

fn node_id_from_matches(matches: &ArgMatches<'static>) -> NodeId {
    matches.value_of("node-id").unwrap().parse().unwrap()
}

fn socket_addrs_from_matches(matches: &ArgMatches<'static>) -> HashMap<NodeId, SocketAddr> {
    let hosts_file_path = matches.value_of("hosts-file").unwrap();
    let string = fs::read_to_string(hosts_file_path).expect("Unable to read file");
    socket_addrs_from_string(string)
}

fn socket_addrs_from_string(string: String) -> HashMap<NodeId, SocketAddr> {
    let mut socket_addrs = HashMap::new();

    for line in string.lines() {
        let components: Vec<&str> = line.split(",").collect();
        let id = components[0].parse().unwrap();
        let socket_addr = components[1].parse().unwrap();

        socket_addrs.insert(id, socket_addr);
    }

    socket_addrs
}

fn color_from_matches(matches: &ArgMatches<'static>) -> Color {
    matches.value_of("color").unwrap().parse().unwrap()
}

fn run_length_from_matches(matches: &ArgMatches<'static>) -> Duration {
    let seconds = matches.value_of("run-length").unwrap().parse().unwrap();
    Duration::from_secs(seconds)
}
