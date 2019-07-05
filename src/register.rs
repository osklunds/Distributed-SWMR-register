
use std::collections::{HashMap, HashSet, BTreeMap};

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


#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn get(&self, node_id: NodeId) -> &Entry<V> {
        self.map.get(&node_id).expect("Trying to get entry in register, but that node id does not exist.")
    }

    pub fn set(&mut self, node_id: NodeId, entry: Entry<V>) {
        if self.map.insert(node_id, entry) == None {
            panic!("Trying to set entry in register, but that node id does not exist.");
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

impl<V> Register<V> {
    fn panic_if_not_same_node_ids(&self, other: &Register<V>) {
        for node_id in self.map.keys() {
            other.map.get(node_id).expect("Comparing eq for two registers with different node ids.");
        }

        for node_id in other.map.keys() {
            self.map.get(node_id).expect("Comparing eq for two registers with different node ids.");
        }
    }
}

impl<V: Display> Display for Register<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sorted_map = BTreeMap::new();
        for (node_id, entry) in self.map.iter() {
            sorted_map.insert(node_id, entry);
        }
        let mut string = String::new();
        for (node_id, entry) in sorted_map.iter() {
            string.push_str(&format!("{}: {}\n", node_id, entry));
        }

        write!(f, "{}", string.trim_end())
    }
}

impl<V: PartialEq> PartialEq for Register<V> {
    fn eq(&self, other: &Self) -> bool {
        if cfg!(debug_assertions) {
            self.panic_if_not_same_node_ids(other);
        }

        self.map == other.map
    }
}

impl<V: PartialOrd> PartialOrd for Register<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if cfg!(debug_assertions) {
            self.panic_if_not_same_node_ids(other);
        }

        let lhs = &self.map;
        let rhs = &other.map;

        let mut lhs_has_one_greater = false;
        let mut rhs_has_one_greater = false;

        for node_id in lhs.keys() {
            let lhs_val = lhs.get(&node_id).unwrap();
            let rhs_val = rhs.get(&node_id).unwrap();

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

    fn register_for_tests() -> Register<String> {
        Register::new(node_ids_for_tests())
    }

    #[test]
    fn test_that_new_contains_provided_node_ids() {
        let reg = register_for_tests();

        for node_id in node_ids_for_tests().iter() {
            assert!(reg.map.contains_key(node_id));
        }
    }

    #[test]
    fn test_that_new_contains_no_other_node_ids() {
        let reg = register_for_tests();

        for node_id in reg.map.keys() {
            assert!(node_ids_for_tests().contains(node_id));
        }
    }

    #[test]
    fn test_that_from_new_timestamps_are_default() {
        let reg = register_for_tests();

        for (_, entry) in reg.map.iter() {
            assert_eq!(entry.ts, default_timestamp());
        }
    }

    #[test]
    fn test_that_get_works_for_existing_node_id() {
        let reg = register_for_tests();

        assert_eq!(*reg.get(1), Entry::new(default_timestamp(), String::default()));
    }

    #[test]
    #[should_panic]
    fn test_that_get_panics_for_non_existing_node_id() {
        let reg = register_for_tests();
        reg.get(5);
    }

    #[test]
    fn test_that_set_works_for_existing_node_id() {
        let mut reg = register_for_tests();
        let entry = Entry::new(10, String::from("Hi"));
        reg.set(1, entry.clone());

        assert_eq!(*reg.get(1), entry);
    }

    #[test]
    fn test_display_register() {
        let mut reg = register_for_tests();
        reg.set(2, Entry::new(7, String::from("Hi")));
        let string = format!("{}", reg);
        let correct = String::from("1: [ts = -1, val = ]\n2: [ts = 7, val = Hi]\n3: [ts = -1, val = ]\n4: [ts = -1, val = ]");

        assert_eq!(string, correct);
    }

    #[test]
    fn test_registers_equal() {
        let reg1 = register_for_tests();
        let reg2 = register_for_tests();

        assert_eq!(reg1, reg2);
    }

    #[test]
    fn test_registers_inequal_entries() {
        let reg1 = register_for_tests();
        let mut reg2 = register_for_tests();
        reg2.set(1, Entry::new(7, String::from("Rust")));

        assert_ne!(reg1, reg2);
    }

    #[test]
    #[should_panic]
    fn test_registers_inequal_node_ids() {
        let reg1 = register_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg2 = Register::new(node_ids);

        assert_ne!(reg1, reg2);
    }

}

