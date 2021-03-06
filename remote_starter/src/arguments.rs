use std::collections::HashSet;

use clap::{App, AppSettings, Arg, ArgMatches};

use commons::arguments;
use commons::node_info::NodeInfo;
use commons::types::Int;

lazy_static! {
    pub static ref ARGUMENTS: Arguments = Arguments::new();
}

#[derive(Debug)]
pub struct Arguments {
    pub hosts_file: String,
    pub node_infos: HashSet<NodeInfo>,
    pub should_write: bool,
    pub number_of_readers: Int,
    pub release_mode_string: String,
    pub print_client_operations_string: String,
    pub run_length_string: String,
    pub install: bool,
}

impl Arguments {
    fn new() -> Arguments {
        let matches = get_matches();

        Arguments {
            hosts_file: arguments::hosts_file_from_matches(&matches),
            node_infos: arguments::node_infos_from_matches(&matches),
            should_write: arguments::should_write_from_matches(
                &matches,
            ),
            number_of_readers: arguments::number_of_readers_from_matches(
                &matches,
            ),
            release_mode_string:
                arguments::release_mode_string_from_matches(&matches),
            print_client_operations_string:
                arguments::print_client_operations_string_from_matches(
                    &matches,
                ),
            run_length_string: arguments::run_length_string_from_matches(
                &matches,
            ),
            install: install_from_matches(&matches),
        }
    }

    pub fn node_infos_for_unique_hosts(&self) -> HashSet<NodeInfo> {
        let mut node_ids_for_unique_hosts = HashSet::new();
        let mut handled_hosts = HashSet::new();

        for node_info in self.node_infos.iter() {
            if handled_hosts.insert(node_info.ip_addr_string()) {
                node_ids_for_unique_hosts.insert(node_info.clone());
            }
        }

        node_ids_for_unique_hosts
    }

    pub fn number_of_nodes(&self) -> Int {
        self.node_infos.len() as Int
    }
}

fn get_matches() -> ArgMatches<'static> {
    App::new("Distributed SWMR register: Remote starter")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .about("A helper utility that starts multiple nodes on remote machines via SSH.")
        .arg(arguments::hosts_file(
            "The file with node ids, addresses, ports, ssh key paths and usernames.",
        ))
        .arg(arguments::should_write())
        .arg(arguments::number_of_readers())
        .arg(arguments::run_length())
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
