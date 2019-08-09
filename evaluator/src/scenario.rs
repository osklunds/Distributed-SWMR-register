
use serde::{Serialize, Deserialize};

use commons::types::Int;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(into = "String")]
#[serde(from = "String")]
pub struct Scenario {
    pub number_of_nodes: Int,
    pub number_of_readers: Int,
    pub number_of_writers: Int
}
// This struct is serialized in String because it's used as a key. And json only allows string keys.

impl Scenario {
    pub fn new(number_of_nodes: Int, number_of_readers: Int, number_of_writers: Int) -> Scenario {
        Scenario {
            number_of_nodes: number_of_nodes,
            number_of_readers: number_of_readers,
            number_of_writers: number_of_writers
        }
    }
}

impl From<Scenario> for String {
    fn from(scenario: Scenario) -> String {
        format!("Scenario,{},{},{}", scenario.number_of_nodes, scenario.number_of_readers, scenario.number_of_writers)
    }
}

impl From<String> for Scenario {
    fn from(string: String) -> Scenario {
        let components: Vec<&str> = string.split(",").collect();
        let scenario_name = components[0];
        let number_of_nodes = components[1].parse().expect("Could not parse number_of_nodes");
        let number_of_readers = components[2].parse().expect("Could not parse number_of_readers");
        let number_of_writers = components[3].parse().expect("Could not parse number_of_writers");

        if scenario_name != "Scenario" {
            panic!("Scenario name doesn't match.");
        }

        Scenario {
            number_of_nodes: number_of_nodes,
            number_of_readers: number_of_readers,
            number_of_writers: number_of_writers
        }
    }
}
