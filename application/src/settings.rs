
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::fs;

use colored::*;
use clap::{Arg, App, ArgMatches};

use crate::register::NodeId;




lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}

#[derive(Debug)]
pub struct Settings {
    pub node_id: NodeId,
    pub socket_addrs: HashMap<NodeId, SocketAddr>,
    pub terminal_color: Color
}

impl Settings {
    fn new() -> Settings {
        let matches = get_matches();
        let color = color_from_matches(&matches);
        let socket_addrs = socket_addrs_from_matches(&matches);
        let node_id = node_id_from_matches(&matches);

        Settings {
            node_id: node_id,
            socket_addrs: socket_addrs,
            terminal_color: color
        }
    }
}

fn get_matches() -> ArgMatches<'static> {
    let colors = &["Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan", "White"];
    App::new("Distributed SWMR registers")
        .version("0.1")
        .author("Oskar Lundström")
        .about("Todo")
        .arg(Arg::with_name("node-id")
            .required(true)
            .help("The integer id of this node instance."))
        .arg(Arg::with_name("hosts-file")
            .required(true)
            .help("The file with host ids, addresses and ports."))
        .arg(Arg::with_name("color")
            .takes_value(true)
            .possible_values(colors)
            .default_value("Black")
            .help("Sets the color of the terminal output"))
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