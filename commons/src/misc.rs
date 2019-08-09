
use colored::Color;
use colored::Color::*;

use crate::types::NodeId;


pub fn color_from_node_id(node_id: NodeId) -> Color {
    let colors = vec![Black, Red, Green, Yellow, Blue, Magenta, Cyan];
    colors[(node_id as usize) % colors.len()]
}
