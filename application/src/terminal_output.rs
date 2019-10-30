use colored::Colorize;

use crate::settings::SETTINGS;

// Prints a string with the color assigned to this node.
pub fn printlnu(string: String) {
    let output_string =
        format!("[Node {}] {}", SETTINGS.node_id(), string);
    println!("{}", output_string.color(SETTINGS.terminal_color()).bold());
}
