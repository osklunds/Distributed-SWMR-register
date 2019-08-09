
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::{Formatter, Display, Result};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

use commons::types::NodeId;

use crate::register::{self, Register};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterArray<V> {
    map: HashMap<NodeId, Register<V>>
}

impl<V: Default + Clone> RegisterArray<V> {
    #[allow(dead_code)]
    pub fn new(node_ids: &HashSet<NodeId>) -> RegisterArray<V> {
        let mut map = HashMap::new();
        for &node_id in node_ids {
            map.insert(node_id, Register::new(register::default_timestamp(), V::default()));
        }

        RegisterArray {
            map: map
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, node_id: NodeId) -> &Register<V> {
        self.map.get(&node_id).expect("Trying to get register in RegisterArray, but that node id does not exist.")
    }

    #[allow(dead_code)]
    pub fn set(&mut self, node_id: NodeId, register: Register<V>) {
        if self.map.insert(node_id, register) == None {
            panic!("Trying to set register in RegisterArray, but that node id does not exist.");
        } 
    }

    #[allow(dead_code)]
    pub fn merge_to_max_from_register_array(&mut self, other: &RegisterArray<V>) {
        for (node_id, register) in self.map.iter_mut() {
            let other_register = other.map.get(node_id).unwrap();
            if other_register > register {
                register.ts = other_register.ts;
                register.val = other_register.val.clone();
            }
        }
    }
}

impl<V> RegisterArray<V> {
    fn panic_if_not_same_node_ids(&self, other: &RegisterArray<V>) {
        for node_id in self.map.keys() {
            other.map.get(node_id).expect("Comparing two RegisterArrays with different node ids.");
        }

        for node_id in other.map.keys() {
            self.map.get(node_id).expect("Comparing two RegisterArrays with different node ids.");
        }
    }
}

impl<V: Display> Display for RegisterArray<V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut sorted_map = BTreeMap::new();
        for (node_id, register) in self.map.iter() {
            sorted_map.insert(node_id, register);
        }
        let mut string = String::new();
        for (node_id, register) in sorted_map.iter() {
            string.push_str(&format!("{}: {}\n", node_id, register));
        }

        write!(f, "{}", string.trim_end())
    }
}

impl<V> PartialEq for RegisterArray<V> {
    fn eq(&self, other: &Self) -> bool {
        if cfg!(debug_assertions) {
            self.panic_if_not_same_node_ids(other);
        }

        self.map == other.map
    }
}

impl<V> PartialOrd for RegisterArray<V> {
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
    use crate::register::Timestamp;

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

    fn register_array_for_tests() -> RegisterArray<String> {
        let mut reg = RegisterArray::new(&node_ids_for_tests());
        for &node_id in node_ids_for_tests().iter() {
            reg.set(node_id, Register::new(timestamp_for_tests(), value_for_tests()));
        }

        reg
    }

    #[test]
    fn test_that_new_contains_provided_node_ids() {
        let reg = register_array_for_tests();

        for node_id in node_ids_for_tests().iter() {
            assert!(reg.map.contains_key(node_id));
        }
    }

    #[test]
    fn test_that_new_contains_no_other_node_ids() {
        let reg = register_array_for_tests();

        for node_id in reg.map.keys() {
            assert!(node_ids_for_tests().contains(node_id));
        }
    }

    #[test]
    fn test_that_from_new_timestamps_are_default() {
        let reg: RegisterArray<String> = RegisterArray::new(&node_ids_for_tests());

        for (_, register) in reg.map.iter() {
            assert_eq!(register.ts, register::default_timestamp());
        }
    }

    #[test]
    fn test_that_get_works_for_existing_node_id() {
        let reg = register_array_for_tests();

        assert_eq!(*reg.get(1), Register::new(timestamp_for_tests(), value_for_tests()));
    }

    #[test]
    #[should_panic]
    fn test_that_get_panics_for_non_existing_node_id() {
        let reg = register_array_for_tests();
        reg.get(5);
    }

    #[test]
    fn test_that_set_works_for_existing_node_id() {
        let mut reg = register_array_for_tests();
        let register = Register::new(timestamp_for_tests(), String::from("Hi"));
        reg.set(1, register.clone());

        assert_eq!(*reg.get(1), register);
    }

    #[test]
    fn test_display_register_array() {
        let mut reg = register_array_for_tests();
        reg.set(2, Register::new(7, String::from("Hi")));
        let string = format!("{}", reg);
        let correct = String::from(format!("1: [ts = {}, val = {}]\n2: [ts = 7, val = Hi]\n3: [ts = {}, val = {}]\n4: [ts = {}, val = {}]", timestamp_for_tests(), value_for_tests(), timestamp_for_tests(), value_for_tests(), timestamp_for_tests(), value_for_tests()));

        assert_eq!(string, correct);
    }

    #[test]
    fn test_register_arrays_equal() {
        let reg1 = register_array_for_tests();
        let reg2 = register_array_for_tests();

        assert_eq!(reg1, reg2);
    }

    #[test]
    fn test_register_arrays_inequal_entries() {
        let reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();
        reg2.set(1, Register::new(7, value_for_tests()));

        assert_ne!(reg1, reg2);
    }

    #[test]
    #[should_panic]
    fn test_register_arrays_eq_inequal_node_ids() {
        let reg1 = register_array_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg2 = RegisterArray::new(&node_ids);

        assert_ne!(reg1, reg2);
    }

    #[test]
    #[should_panic]
    fn test_register_arrays_ord_inequal_node_ids() {
        let reg1 = register_array_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg2 = RegisterArray::new(&node_ids);

        assert_eq!(reg1 >= reg2, false);
    }

    #[test]
    fn test_register_arrays_leq_for_equal() {
        let reg1 = register_array_for_tests();
        let reg2 = register_array_for_tests();

        assert!(reg2 <= reg1);
    }

    #[test]
    fn test_register_arrays_leq_for_one_less_register() {
        let reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();
        reg2.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));

        assert!(reg2 <= reg1);
    }

    #[test]
    fn test_register_arrays_leq_for_one_less_and_one_greater_register() {
        let reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();
        reg2.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        assert!(!(reg2 <= reg1));
    }

    #[test]
    fn test_register_arrays_le_for_equal() {
        let reg1 = register_array_for_tests();
        let reg2 = register_array_for_tests();

        assert!(!(reg2 < reg1));
    }

    #[test]
    fn test_register_arrays_le_for_one_less_register() {
        let reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();
        reg2.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));

        assert!(reg2 < reg1);
    }

    #[test]
    fn test_merge_to_max_overwrites_lower() {
        let mut reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();

        reg1.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        reg1.merge_to_max_from_register_array(&reg2);

        assert_eq!(*reg1.get(1), Register::new(timestamp_for_tests(), value_for_tests()));
    }

    #[test]
    fn test_merge_to_max_includes_higher() {
        let mut reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();

        reg1.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        reg1.merge_to_max_from_register_array(&reg2);

        assert_eq!(*reg1.get(2), Register::new(timestamp_for_tests() + 1, value_for_tests()));
    }

    #[test]
    fn test_merge_to_max_keeps_equals_intact() {
        let mut reg1 = register_array_for_tests();
        let mut reg2 = register_array_for_tests();

        reg1.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        reg1.merge_to_max_from_register_array(&reg2);

        assert_eq!(*reg1.get(3), Register::new(timestamp_for_tests(), value_for_tests()));
    }
}