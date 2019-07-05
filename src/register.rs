
use std::collections::{HashMap, HashSet};

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;

use std::cmp;
use std::cmp::Ordering;

use std::hash::Hash;
use std::hash::Hasher;

use serde::{Serialize, Deserialize};


type Timestamp = i32;
type NodeId = i32;

fn default_timestamp() -> Timestamp {
    -1
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Register<V> {
    map: HashMap<NodeId, Entry<V>>
}

impl<V: Default + Clone> Register<V> {
    pub fn new(node_ids: HashSet<NodeId>) -> Register<V> {
        let mut map = HashMap::new();
        for node_id in node_ids {
            map.insert(node_id, Entry::new(-1, V::default()));
        }

        Register {
            map: map
        }
    }

    pub fn get(&self, node_id: i32) -> Option<&Entry<V>> {
        self.map.get(&node_id)
    }

    pub fn set(&mut self, node_id: i32, entry: Entry<V>) {
        if self.map.insert(node_id, entry) == None {
            panic!("Trying to set entry in register, but that node_id does not exist.");
        } 
    }

    pub fn merge_to_max_from_register(&mut self, other: &Register<V>) {
        // This is an ineffective hack for now

        let mut new_map = HashMap::new();

        for node_id in self.map.keys() {
            let my_val = self.map.get(node_id).unwrap();
            let other_val = other.map.get(node_id).unwrap();
            
            new_map.insert(*node_id, cmp::max(my_val.clone(), other_val.clone()));
        }

        self.map = new_map;
    }
}

impl<V: Display> Display for Register<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for (node_id, entry) in self.map.iter() {
            string.push_str(&format!("{}: {}", node_id, entry));
        }

        write!(f, "{}", string)
    }
}

impl<V: PartialOrd> PartialOrd for Register<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = &self.map;
        let rhs = &other.map;

        let mut lhs_has_one_greater = false;
        let mut rhs_has_one_greater = false;

        for node_id in lhs.keys() {
            let lhs_val = lhs.get(&node_id).unwrap();
            let rhs_val = rhs.get(&node_id).expect("Attempting to compare registers with different keys.");

            if lhs_val > rhs_val {
                lhs_has_one_greater = true;
            } else if lhs_val < rhs_val {
                rhs_has_one_greater = true;
            }
        }
        
        if lhs_has_one_greater && !rhs_has_one_greater {
            Some(Ordering::Less)
        } else if !lhs_has_one_greater && rhs_has_one_greater {
            Some(Ordering::Greater)
        } else if !lhs_has_one_greater && !rhs_has_one_greater {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry<V> {
    pub ts: Timestamp,
    pub val: V
}

impl<V> Entry<V> {
    pub fn new(ts: Timestamp, val: V) -> Entry<V> {
        Entry {
            ts: ts,
            val: val
        }
    }
}

impl<V: Display> Display for Entry<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ts = {}, val = {}]", self.ts, self.val)
    }
}

impl<V> PartialEq for Entry<V> {
    fn eq(&self, other: &Self) -> bool {
        self.ts == other.ts
    }
}

impl<V> Eq for Entry<V> {}

impl<V> PartialOrd for Entry<V> {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        self.ts.partial_cmp(&other.ts)
    }
}

impl<V> Ord for Entry<V> {
    fn cmp(&self, other:&Self) -> Ordering {
        self.ts.cmp(&other.ts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_ids_for_tests() -> HashSet<NodeId> {
        let mut node_ids = HashSet::new();
        node_ids.insert(1);
        node_ids.insert(2);
        node_ids.insert(3);
        node_ids.insert(4);
        node_ids
    }

    #[test]
    fn test_that_new_contains_provided_node_ids() {
        let reg: Register<String> = Register::new(node_ids_for_tests());

        for node_id in node_ids_for_tests().iter() {
            assert!(reg.map.contains_key(node_id));
        }
    }

    #[test]
    fn test_that_new_contains_no_other_node_ids() {
        let reg: Register<String> = Register::new(node_ids_for_tests());

        for node_id in reg.map.keys() {
            assert!(node_ids_for_tests().contains(node_id));
        }
    }

    #[test]
    fn test_that_from_new_timestamps_are_default() {
        let reg: Register<String> = Register::new(node_ids_for_tests());

        for (_, entry) in reg.map.iter() {
            assert_eq!(entry.ts, default_timestamp());
        }
    }



}

