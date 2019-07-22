
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::run_result::*;


pub type EvaluationResults = HashMap<RunScenario, ScenarioResults>;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct RunScenario {
    pub number_of_nodes: usize,
    pub number_of_readers: usize,
    pub number_of_writers: usize
}

impl RunScenario {
    pub fn new() -> RunScenario {
        RunScenario {
            number_of_nodes: 0,
            number_of_readers: 0,
            number_of_writers: 0
        }
    }
}