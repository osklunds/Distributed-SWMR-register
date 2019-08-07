
use clap::{Arg, App, ArgMatches, AppSettings};

lazy_static! {
    pub static ref ARGUMENTS: Arguments = Arguments::new();
}


pub struct Arguments {
    pub number_of_nodes: i32,
    pub number_of_writers: i32,
    pub number_of_readers: i32,
    pub release_mode_string: String,
    pub print_client_operations_string: String,
    pub run_length_string: String,
    pub record_evaluation_info_string: String
}

impl Arguments {
    fn new() -> Arguments {
        let matches = get_matches();

        let number_of_nodes = number_of_nodes_from_matches(&matches);
        let number_of_writers = number_of_writers_from_matches(&matches);
        let number_of_readers = number_of_readers_from_matches(&matches);
        let release_mode_string = release_mode_string_from_matches(&matches);
        let print_client_operations_string = print_client_operations_string_from_matches(&matches);
        let run_length_string = run_length_string_from_matches(&matches);
        let record_evaluation_info_string = record_evaluation_info_string_from_matches(&matches);

        Arguments {
            number_of_nodes: number_of_nodes,
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
    App::new("Distributed-SWMR-registers: Local starter")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .about("A helper utility that starts multiple nodes on your local computer.")

        .arg(Arg::with_name("number-of-nodes")
            .required(true)
            .takes_value(true)
            .short("n")
            .long("number-of-nodes")
            .help("The number of local nodes to run."))

        .arg(Arg::with_name("number-of-writers")
            .required(false)
            .takes_value(true)
            .default_value("0")
            .short("w")
            .long("number-of-writers")
            .help("The number of nodes that should write."))

        .arg(Arg::with_name("number-of-readers")
            .required(false)
            .takes_value(true)
            .default_value("0")
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
            .required(false)
            .takes_value(true)
            .default_value("0")
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

fn number_of_nodes_from_matches(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-nodes").unwrap().parse().unwrap()
}

fn number_of_writers_from_matches(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-writers").unwrap().parse().unwrap()
}

fn number_of_readers_from_matches(matches: &ArgMatches<'static>) -> i32 {
    matches.value_of("number-of-readers").unwrap().parse().unwrap()
}

fn release_mode_string_from_matches(matches: &ArgMatches<'static>) -> String {
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

fn record_evaluation_info_string_from_matches(matches: &ArgMatches<'static>) -> String {
    match matches.is_present("record-evaluation-info") {
        true  => "--record-evaluation-info".to_string(),
        false => "".to_string()
    }
}
