use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, Weak};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use commons::types::{Int, NodeId};

use crate::data_types::register::Register;
use crate::data_types::register_array::*;
use crate::data_types::timestamp::Timestamp;
use crate::mediator::Med;
use crate::messages::*;
use crate::terminal_output::printlnu;

pub struct Quorum {
    acking_processors: Mutex<HashSet<NodeId>>,
    accessing: Mutex<bool>,
    majority_reached: Condvar,
    number_of_nodes: Int,
}

impl Quorum {
    pub fn new(number_of_nodes: Int) -> Quorum {
        Quorum {
            acking_processors: Mutex::new(HashSet::new()),
            accessing: Mutex::new(false),
            majority_reached: Condvar::new(),
            number_of_nodes: number_of_nodes,
        }
    }

    pub fn acking_processors(&self) -> &Mutex<HashSet<NodeId>> {
        &self.acking_processors
    }

    pub fn accessing(&self) -> &Mutex<bool> {
        &self.accessing
    }

    pub fn majority_reached(&self) -> &Condvar {
        &self.majority_reached
    }

    pub fn is_idle(&self) -> bool {
        self.acking_processors.lock().unwrap().is_empty()
            && !*self.accessing.lock().unwrap()
    }

    pub fn notify_if_has_ack_from_majority(&self) {
        if self.has_ack_from_majority() {
            let mut accessing =
                self.accessing.lock().unwrap();
            *accessing = false;
            self.majority_reached.notify_one();
        }
    }

    fn has_ack_from_majority(&self) -> bool {
        let acking_processors =
            self.acking_processors.lock().unwrap();

        acking_processors.len() as Int
            >= self.number_of_nodes_in_a_majority()
    }

    fn number_of_nodes_in_a_majority(&self) -> Int {
        self.number_of_nodes / 2 + 1
    }
}
