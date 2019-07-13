
use std::process::Command;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use clap::{Arg, App, ArgMatches};
use colored::Color;
use colored::Color::*;


fn main() {
    let matches = get_matches();
    let number_of_nodes = number_of_nodes(&matches);
    let number_of_writers = number_of_writers(&matches);
    let number_of_readers = number_of_readers(&matches);
    let release_mode_string = release_mode_string(&matches);
    let print_client_operations_string = print_client_operations_string(&matches);
    let run_length = matches.value_of("run-length").unwrap();
    let record_evaluation_info_string = record_evaluation_info_string(&matches);

    create_hosts_file(number_of_nodes);

    let mut build_process = Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("cargo build {} --manifest-path ../application/Cargo.toml", release_mode_string))
        .spawn()
        .expect("failed to execute process");

    build_process.wait().unwrap();

    let mut child_processes = Vec::new();
    for node_id in 1..number_of_nodes+1 {
        let color = color_from_node_id(node_id);
        let write_string = match node_id <= number_of_writers {
            true  => "--write",
            false => ""
        };
        let read_string = match node_id <= number_of_readers {
            true => "--read",
            false => ""
        };

        let command_string = format!("cargo run {} --manifest-path ../application/Cargo.toml -- {} hosts.txt {} {:?} {} {} {} {}", release_mode_string, node_id, run_length, color, print_client_operations_string, record_evaluation_info_string, write_string, read_string);

        let child_process = Command::new("/bin/bash")
                .arg("-c")
                .arg(command_string)
                .spawn()
                .expect("failed to execute process");

        child_processes.push(child_process);
    }
 
    for child_process in child_processes.iter_mut() {
        child_process.wait().unwrap();
    }
}

fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Local starter")
        .version("0.1")
        .author("Oskar Lundström")
        .about("Todo")

        .arg(Arg::with_name("number-of-nodes")
            .required(true)
            .takes_value(true)
            .short("n")
            .long("number-of-nodes")
            .help("The number of local nodes to run."))

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

        .arg(Arg::with_name("optimize")
            .takes_value(false)
            .short("o")
            .long("optimize")
            .help("With this option, cargo will build/run in release mode. This uses optimizations and yields higher performance."))

        .arg(Arg::with_name("print-client-operations")
            .short("p")
            .long("print-client-operations")
            .takes_value(false)
            .help("Print when a read/write operation starts/ends. If not included, the performance might be slightly higher."))

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

fn create_hosts_file(number_of_nodes: i32) {
    let correct_string = hosts_file_string(number_of_nodes);
    let file_path = Path::new("hosts.txt");
    if file_path.exists() {
        if let Ok(existing_string) = fs::read_to_string(file_path) {
            if existing_string == correct_string {
                return;
            }
        }

        fs::remove_file(file_path).expect("Could not remove existing hosts.txt file");
    }

    fs::write(file_path, correct_string).expect("Could not write new hosts.txt file.");
}

fn hosts_file_string(number_of_nodes: i32) -> String {
    let mut string = String::new();
    let port_offset = 62000;

    for node_id in 1..number_of_nodes+1 {
        string.push_str(&format!("{},127.0.0.1:{}\n", node_id, node_id + port_offset));
    }

    string
}

fn color_from_node_id(node_id: i32) -> Color {
    let colors = vec![Black, Red, Green, Yellow, Blue, Magenta, Cyan];
    colors[(node_id as usize) % 7]
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

fn print_client_operations_string(matches: &ArgMatches<'static>) -> String {
    if matches.is_present("print-client-operations") {
        "--print-client-operations".to_string()
    } else {
        "".to_string()
    }
}

fn record_evaluation_info_string(matches: &ArgMatches<'static>) -> String {
    if matches.is_present("record-evaluation-info") {
        "record-evaluation-info".to_string()
    } else {
        "".to_string()
    }
}
