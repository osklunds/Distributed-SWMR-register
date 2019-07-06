
use colored::*;
use clap::{Arg, App, SubCommand};





lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}

#[derive(Debug)]
pub struct Settings {
    pub terminal_color: Color
}

impl Settings {
    fn new() -> Settings {
        let colors = &["Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan", "White"];
        let matches = App::new("Distributed SWMR registers")
                          .version("0.1")
                          .author("Oskar Lundström")
                          .about("Todo")
                          .arg(Arg::with_name("hosts-file")
                                .required(true)
                                .help("The file with host ids, addresses and ports."))
                          .arg(Arg::with_name("color")
                                .takes_value(true)
                                .possible_values(colors)
                                .help("Sets the color of the terminal output"))
                        .get_matches();

        let color;

        if matches.is_present("color") {
            color = Color::from(matches.value_of("color").unwrap());
        } else {
            color = Color::Black;
        }     

        Settings {
            terminal_color: color
        }
    }
}