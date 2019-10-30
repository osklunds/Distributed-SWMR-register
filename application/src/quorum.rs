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
}

impl Quorum {
    pub fn new() -> Quorum {
        Quorum {
            acking_processors: Mutex::new(HashSet::new()),
            accessing: Mutex::new(false),
            majority_reached: Condvar::new()
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
        self.acking_processors.lock().unwrap().is_empty() &&
        !*self.accessing.lock().unwrap()
    }
}