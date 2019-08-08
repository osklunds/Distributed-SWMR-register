
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::{Formatter, Display, Result};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

use commons::types::NodeId;

use crate::entry::{self, Entry};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Register<V> {
    map: HashMap<NodeId, Entry<V>>
}

impl<V: Default + Clone> Register<V> {
    #[allow(dead_code)]
    pub fn new(node_ids: &HashSet<NodeId>) -> Register<V> {
        let mut map = HashMap::new();
        for &node_id in node_ids {
            map.insert(node_id, Entry::new(entry::default_timestamp(), V::default()));
        }

        Register {
            map: map
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, node_id: NodeId) -> &Entry<V> {
        self.map.get(&node_id).expect("Trying to get entry in register, but that node id does not exist.")
    }

    #[allow(dead_code)]
    pub fn set(&mut self, node_id: NodeId, entry: Entry<V>) {
        if self.map.insert(node_id, entry) == None {
            panic!("Trying to set entry in register, but that node id does not exist.");
        } 
    }

    #[allow(dead_code)]
    pub fn merge_to_max_from_register(&mut self, other: &Register<V>) {
        for (node_id, entry) in self.map.iter_mut() {
            let other_entry = other.map.get(node_id).unwrap();
            if other_entry > entry {
                entry.ts = other_entry.ts;
                entry.val = other_entry.val.clone();
            }
        }
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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

impl<V> PartialEq for Register<V> {
    fn eq(&self, other: &Self) -> bool {
        if cfg!(debug_assertions) {
            self.panic_if_not_same_node_ids(other);
        }

        self.map == other.map
    }
}

impl<V> PartialOrd for Register<V> {
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

        let eq = !lhs_has_one_greater && !rhs_has_one_greater;

        if eq {
            Some(Ordering::Equal)
        } else if lhs_has_one_greater && !rhs_has_one_greater && !eq {
            Some(Ordering::Greater)
        } else if !lhs_has_one_greater && rhs_has_one_greater && !eq {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::Timestamp;

    fn node_ids_for_tests() -> HashSet<NodeId> {
        let mut node_ids = HashSet::new();
        node_ids.insert(1);
        node_ids.insert(2);
        node_ids.insert(3);
        node_ids.insert(4);
        node_ids
    }

    fn timestamp_for_tests() -> Timestamp {
        10
    }

    fn value_for_tests() -> String {
        String::from("Rust")
    }

    fn register_for_tests() -> Register<String> {
        let mut reg = Register::new(&node_ids_for_tests());
        for &node_id in node_ids_for_tests().iter() {
            reg.set(node_id, Entry::new(timestamp_for_tests(), value_for_tests()));
        }

        reg
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
        let reg: Register<String> = Register::new(&node_ids_for_tests());

        for (_, entry) in reg.map.iter() {
            assert_eq!(entry.ts, entry::default_timestamp());
        }
    }

    #[test]
    fn test_that_get_works_for_existing_node_id() {
        let reg = register_for_tests();

        assert_eq!(*reg.get(1), Entry::new(timestamp_for_tests(), value_for_tests()));
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
        let entry = Entry::new(timestamp_for_tests(), String::from("Hi"));
        reg.set(1, entry.clone());

        assert_eq!(*reg.get(1), entry);
    }

    #[test]
    fn test_display_register() {
        let mut reg = register_for_tests();
        reg.set(2, Entry::new(7, String::from("Hi")));
        let string = format!("{}", reg);
        let correct = String::from(format!("1: [ts = {}, val = {}]\n2: [ts = 7, val = Hi]\n3: [ts = {}, val = {}]\n4: [ts = {}, val = {}]", timestamp_for_tests(), value_for_tests(), timestamp_for_tests(), value_for_tests(), timestamp_for_tests(), value_for_tests()));

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
        reg2.set(1, Entry::new(7, value_for_tests()));

        assert_ne!(reg1, reg2);
    }

    #[test]
    #[should_panic]
    fn test_registers_eq_inequal_node_ids() {
        let reg1 = register_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg2 = Register::new(&node_ids);

        assert_ne!(reg1, reg2);
    }

    #[test]
    #[should_panic]
    fn test_registers_ord_inequal_node_ids() {
        let reg1 = register_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg2 = Register::new(&node_ids);

        assert_eq!(reg1 >= reg2, false);
    }

    #[test]
    fn test_registers_leq_for_equal() {
        let reg1 = register_for_tests();
        let reg2 = register_for_tests();

        assert!(reg2 <= reg1);
    }

    #[test]
    fn test_registers_leq_for_one_less_entry() {
        let reg1 = register_for_tests();
        let mut reg2 = register_for_tests();
        reg2.set(1, Entry::new(timestamp_for_tests() - 1, value_for_tests()));

        assert!(reg2 <= reg1);
    }

    #[test]
    fn test_registers_leq_for_one_less_and_one_greater_entry() {
        let reg1 = register_for_tests();
        let mut reg2 = register_for_tests();
        reg2.set(1, Entry::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Entry::new(timestamp_for_tests() + 1, value_for_tests()));

        assert!(!(reg2 <= reg1));
    }

    #[test]
    fn test_registers_le_for_equal() {
        let reg1 = register_for_tests();
        let reg2 = register_for_tests();

        assert!(!(reg2 < reg1));
    }

    #[test]
    fn test_registers_le_for_one_less_entry() {
        let reg1 = register_for_tests();
        let mut reg2 = register_for_tests();
        reg2.set(1, Entry::new(timestamp_for_tests() - 1, value_for_tests()));

        assert!(reg2 < reg1);
    }

    #[test]
    fn test_merge_to_max_overwrites_lower() {
        let mut reg1 = register_for_tests();
        let mut reg2 = register_for_tests();

        reg1.set(1, Entry::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Entry::new(timestamp_for_tests() + 1, value_for_tests()));

        reg1.merge_to_max_from_register(&reg2);

        assert_eq!(*reg1.get(1), Entry::new(timestamp_for_tests(), value_for_tests()));
    }

    #[test]
    fn test_merge_to_max_includes_higher() {
        let mut reg1 = register_for_tests();
        let mut reg2 = register_for_tests();

        reg1.set(1, Entry::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Entry::new(timestamp_for_tests() + 1, value_for_tests()));

        reg1.merge_to_max_from_register(&reg2);

        assert_eq!(*reg1.get(2), Entry::new(timestamp_for_tests() + 1, value_for_tests()));
    }

    #[test]
    fn test_merge_to_max_keeps_equals_intact() {
        let mut reg1 = register_for_tests();
        let mut reg2 = register_for_tests();

        reg1.set(1, Entry::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Entry::new(timestamp_for_tests() + 1, value_for_tests()));

        reg1.merge_to_max_from_register(&reg2);

        assert_eq!(*reg1.get(3), Entry::new(timestamp_for_tests(), value_for_tests()));
    }
}