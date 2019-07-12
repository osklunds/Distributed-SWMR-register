
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use std::collections::{HashMap, HashSet};
//use std::iter::FromIterator;
use std::net::SocketAddr;
use std::net::IpAddr::V4;
use std::net::ToSocketAddrs;

use std::process::Command;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;


use clap::{Arg, App, ArgMatches};


type NodeId = i32;

#[derive(PartialEq, Eq, Hash)]
pub struct NodeInfo {
    pub node_id: NodeId,
    pub socket_addr: SocketAddr,
    pub key_path: String,
    pub username: String
}


fn main() {
    let matches = get_matches();
    let node_infos = node_infos_from_matches(&matches);

    for node_info in node_infos.iter() {
        if let V4(ip_addr) = node_info.socket_addr.ip() {
            let mut install_process = Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("ssh -i {} {}@{} \"curl https://sh.rustup.rs -sSf > rustup.sh;sh rustup.sh -y; exit\"", node_info.key_path, node_info.username, ip_addr))
                .spawn()
                .expect("failed to execute the install process");

            install_process.wait().unwrap();

        }
    }

    /*

    let mut build_process = Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("cargo build {} --manifest-path ../application/Cargo.toml", release_mode_string))
        .spawn()
        .expect("failed to execute process");

    build_process.wait().unwrap();

    let mut child_processes = Vec::new();
    for node_id in 1..number_of_nodes+1 {
        let color = color_from_node_id(node_id);
        let child_process = Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("cargo run {} --manifest-path ../application/Cargo.toml -- {} hosts.txt {:?}", release_mode_string, node_id, color))
                .spawn()
                .expect("failed to execute process");

        child_processes.push(child_process);
    }
 
    for child_process in child_processes.iter_mut() {
        child_process.wait().unwrap();
    }

    */
}

fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Local starter")
        .version("0.1")
        .author("Oskar Lundström")
        .about("Todo")

        .arg(Arg::with_name("hosts-file")
            .required(true)
            .takes_value(true)
            .help("The file with host ids, addresses and ports."))

        .arg(Arg::with_name("number-of-writers")
            .short("w")
            .long("number-of-writers")
            .takes_value(true)
            .help("The number of nodes that should write."))

        .arg(Arg::with_name("number-of-readers")
            .short("r")
            .long("number-of-readers")
            .takes_value(true)
            .help("The number of nodes that should read."))

        .arg(Arg::with_name("optimize")
            .short("o")
            .long("optimize")
            .takes_value(false)
            .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance."))

        .get_matches()
}

fn number_of_nodes(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-nodes").unwrap().parse().unwrap()
}

fn number_of_writers(matches: &ArgMatches<'static>) -> i32 {
    if let Some(number_of_writers) = matches.value_of("number-of-writers") {
        number_of_writers.parse().unwrap()
    } else {
        0
    }
}

fn number_of_readers(matches: &ArgMatches<'static>) -> i32 {
    if let Some(number_of_readers) = matches.value_of("number-of-readers") {
        number_of_readers.parse().unwrap()
    } else {
        0
    }
}

fn release_mode_string(matches: &ArgMatches<'static>) -> String {
    if release_mode(matches) {
        String::from("--release")
    } else {
        String::from("")
    }
}

fn release_mode(matches: &ArgMatches<'static>) -> bool {
    matches.is_present("optimize")
}

fn node_infos_from_matches(matches: &ArgMatches<'static>) -> HashSet<NodeInfo> {
    let hosts_file_path = matches.value_of("hosts-file").unwrap();
    let string = fs::read_to_string(hosts_file_path).expect("Unable to read file");
    socket_addrs_and_key_paths_from_string(string)
}

fn socket_addrs_and_key_paths_from_string(string: String) -> HashSet<NodeInfo> {
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


