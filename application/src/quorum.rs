
use std::collections::HashSet;
use std::sync::{Condvar, Mutex};

use commons::types::{Int, NodeId};

//use crate::terminal_output::printlnu;

pub struct Quorum {
    acking_nodes: Mutex<HashSet<NodeId>>,
    accessing: Mutex<bool>,
    majority_reached: Condvar,
    number_of_nodes: Int,
}

impl Quorum {
    pub fn new(number_of_nodes: Int) -> Quorum {
        Quorum {
            acking_nodes: Mutex::new(HashSet::new()),
            accessing: Mutex::new(false),
            majority_reached: Condvar::new(),
            number_of_nodes: number_of_nodes,
        }
    }

    pub fn insert_node_to_acking_nodes(&self, node_id: NodeId) {
        let mut acking_nodes = self.acking_nodes.lock().unwrap();
        acking_nodes.insert(node_id);
    }

    pub fn accessing(&self) -> &Mutex<bool> {
        &self.accessing
    }

    pub fn majority_reached(&self) -> &Condvar {
        &self.majority_reached
    }

    pub fn is_idle(&self) -> bool {
        self.acking_nodes.lock().unwrap().is_empty()
            && !*self.accessing.lock().unwrap()
    }

    pub fn notify_if_has_ack_from_majority(&self) {
        if self.has_ack_from_majority() {
            let mut acking_nodes = self.acking_nodes.lock().unwrap();
            acking_nodes.clear();

            let mut accessing = self.accessing.lock().unwrap();
            *accessing = false;

            self.majority_reached.notify_one();
        }
    }

    fn has_ack_from_majority(&self) -> bool {
        let acking_nodes = self.acking_nodes.lock().unwrap();

        acking_nodes.len() as Int >= self.number_of_nodes_in_a_majority()
    }

    fn number_of_nodes_in_a_majority(&self) -> Int {
        self.number_of_nodes / 2 + 1
    }

    #[cfg(test)]
    pub fn acking_nodes(&self) -> &Mutex<HashSet<NodeId>> {
        &self.acking_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_nodes_in_a_majority() {
        let number_of_nodes_in_a_majority = vec![1, 2, 2, 3, 3, 4, 4];

        for i in 0..7 {
            let number_of_nodes = i+1;
            let quorum = Quorum::new(number_of_nodes);

            assert_eq!(quorum.number_of_nodes_in_a_majority(), number_of_nodes_in_a_majority[i as usize]);
        }
    }
}
