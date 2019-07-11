
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::net::SocketAddr;
use std::fs;


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
    print_start_end_of_client_operations: bool
}

impl Settings {
    fn new() -> Settings {
        let matches = get_matches();
        let color = color_from_matches(&matches);
        let socket_addrs = socket_addrs_from_matches(&matches);
        let node_id = node_id_from_matches(&matches);
        let print_start_end_of_client_operations = print_start_end_from_matches(&matches);


        Settings {
            node_id: node_id,
            socket_addrs: socket_addrs,
            terminal_color: color,
            print_start_end_of_client_operations: print_start_end_of_client_operations
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

    pub fn print_start_end_of_client_operations(&self) -> bool {
        self.print_start_end_of_client_operations
    }

    pub fn number_of_nodes(&self) -> usize {
        self.socket_addrs.len()
    }

    pub fn node_ids(&self) -> HashSet<NodeId> {
        HashSet::from_iter(self.socket_addrs.keys().map(|id| *id))
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

        .arg(Arg::with_name("color")
            .takes_value(true)
            .possible_values(colors)
            .default_value("Black")
            .help("Sets the color of the terminal output"))

        .arg(Arg::with_name("print-start-end")
            .short("p")
            .long("print-start-end")
            .takes_value(false)
            .help("Print start/end when a client operation starts respectively ends."))

        .get_matches()
}

fn color_from_matches(matches: &ArgMatches<'static>) -> Color {
    matches.value_of("color").unwrap().parse().unwrap()
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

fn node_id_from_matches(matches: &ArgMatches<'static>) -> NodeId {
    matches.value_of("node-id").unwrap().parse().unwrap()
}

fn print_start_end_from_matches(matches: &ArgMatches<'static>) -> bool {
    matches.is_present("print-start-end")
}