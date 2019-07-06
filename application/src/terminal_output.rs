
use colored::Colorize;

use crate::settings::SETTINGS;

pub fn prin(string: &str) {
    println!("{}", string.color(SETTINGS.terminal_color));
}