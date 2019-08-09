
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::{Formatter, Display, Result};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

use commons::types::NodeId;

use super::vector::Vector;
use super::timestamp::Timestamp;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VectorClock {
    vector: Vector<Timestamp>
}

impl VectorClock {
    pub fn new(node_ids: &HashSet<NodeId>) -> VectorClock {
        VectorClock {
            vector: Vector::new(node_ids)
        }
    }

    pub fn get(&self, node_id: NodeId) -> &Timestamp {
        self.vector.get(node_id)
    }

    pub fn set(&mut self, node_id: NodeId, timestamp: Timestamp) {
        self.vector.set(node_id, timestamp);
    }

    pub fn merge_to_max_from_vector_clock(&mut self, other: &VectorClock) {
        self.vector.merge_to_max_from_vector(&other.vector);
    }
}

impl Display for VectorClock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.vector.fmt(f)
    }
}

impl PartialEq for VectorClock {
    fn eq(&self, other: &Self) -> bool {
        self.vector.eq(&other.vector)
    }
}

impl PartialOrd for VectorClock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.vector.partial_cmp(&other.vector)
    }
}