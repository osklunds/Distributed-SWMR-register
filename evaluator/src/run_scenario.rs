
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::run_result::*;


pub type EvaluationResults = HashMap<Scenario, ScenarioResults>;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Scenario {
    pub number_of_nodes: usize,
    pub number_of_readers: usize,
    pub number_of_writers: usize
}

impl Scenario {
    pub fn new(number_of_nodes: usize, number_of_readers: usize, number_of_writers: usize) -> Scenario {
        Scenario {
            number_of_nodes: number_of_nodes,
            number_of_readers: number_of_readers,
            number_of_writers: number_of_writers
        }
    }
}