
use std::collections::HashSet;

use clap::{Arg, App, ArgMatches, AppSettings};

use commons::node_info::NodeInfo;
use commons::types::Int;
use commons::arguments;

lazy_static! {
    pub static ref ARGUMENTS: Arguments = Arguments::new();
}


#[derive(Debug)]
pub struct Arguments {
    pub hosts_file: String,
    pub node_infos: HashSet<NodeInfo>,
    pub number_of_writers: Int,
    pub number_of_readers: Int,
    pub release_mode_string: String,
    pub print_client_operations_string: String,
    pub run_length_string: String,
    pub record_evaluation_info_string: String,
    pub install: bool
}

impl Arguments {
    fn new() -> Arguments {
        let matches = get_matches();

        Arguments {
            hosts_file: arguments::hosts_file_from_matches(&matches),
            node_infos: arguments::node_infos_from_matches(&matches),
            number_of_writers: arguments::number_of_writers_from_matches(&matches),
            number_of_readers: arguments::number_of_readers_from_matches(&matches),
            release_mode_string: arguments::release_mode_string_from_matches(&matches),
            print_client_operations_string: arguments::print_client_operations_string_from_matches(&matches),
            run_length_string: arguments::run_length_string_from_matches(&matches),
            record_evaluation_info_string: arguments::record_evaluation_info_string_from_matches(&matches),
            install: install_from_matches(&matches)
        }
    }
}

fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR registers: Remote starter")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .about("A helper utility that starts multiple nodes on remote machines via SSH.")

        .arg(arguments::hosts_file("The file with node ids, addresses, ports, ssh key paths and usernames."))
        .arg(arguments::number_of_writers())
        .arg(arguments::number_of_readers())
        .arg(arguments::run_length())
        .arg(arguments::record_evaluation_info())
        .arg(arguments::optimize())
        .arg(install_argument())
        .arg(arguments::print_client_operations())

        .get_matches()
}

fn install_argument() -> Arg<'static, 'static> {
    Arg::with_name("install")
        .takes_value(false)
        .short("i")
        .long("install")
        .help("With this option, Rust will be installed, the source code and configuration files will be uploaded and the application will be built. Without this option, the application will be launched.")
}

fn install_from_matches(matches: &ArgMatches<'static>) -> bool {
    matches.is_present("install")
}