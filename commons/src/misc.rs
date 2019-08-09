
use colored::Color;
use colored::Color::*;

use crate::types::NodeId;


pub fn color_from_node_id(node_id: NodeId) -> Color {
    let colors = vec![Black, Red, Green, Yellow, Blue, Magenta, Cyan];
    colors[(node_id as usize) % colors.len()]
}

pub fn run_result_file_name_from_node_id(node_id: NodeId) -> String {
    format!("node{:0>6}.eval", node_id)
}
