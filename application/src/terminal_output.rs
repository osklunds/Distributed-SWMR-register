
use colored::Colorize;

use crate::settings::SETTINGS;

pub fn printlnu(string: &str) {
    let output_string = format!("[Node {}] {}", SETTINGS.node_id, string);
    println!("{}", output_string.color(SETTINGS.terminal_color).bold());
}

