
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::{Formatter, Display, Result};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

use commons::types::NodeId;

use crate::register::{self, Register};
use crate::vector::Vector;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterArray<V> {
    vector: Vector<Register<V>>
}

impl<V: Default + Clone> RegisterArray<V> {
    pub fn new(node_ids: &HashSet<NodeId>) -> RegisterArray<V> {
        RegisterArray {
            vector: Vector::new(node_ids)
        }
    }

    pub fn get(&self, node_id: NodeId) -> &Register<V> {
        self.vector.get(node_id)
    }

    pub fn set(&mut self, node_id: NodeId, register: Register<V>) {
        self.vector.set(node_id, register);
    }

    pub fn merge_to_max_from_register_array(&mut self, other: &RegisterArray<V>) {
        self.vector.merge_to_max_from_vector(&other.vector);
    }
}

impl<V: Display> Display for RegisterArray<V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.vector.fmt(f)
    }
}

impl<V> PartialEq for RegisterArray<V> {
    fn eq(&self, other: &Self) -> bool {
        self.vector.eq(&other.vector)
    }
}

impl<V> PartialOrd for RegisterArray<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.vector.partial_cmp(&other.vector)
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
        let mut reg_array = RegisterArray::new(&node_ids_for_tests());
        for &node_id in node_ids_for_tests().iter() {
            reg_array.set(node_id, Register::new(timestamp_for_tests(), value_for_tests()));
        }

        reg_array
    } 
    /*
    #[test]
    fn test_that_from_new_timestamps_are_default() {
        let reg_array: RegisterArray<String> = RegisterArray::new(&node_ids_for_tests());

        for (_, register) in reg_array.map.iter() {
            assert_eq!(register.ts, register::default_timestamp());
        }
    }*/

    #[test]
    fn test_that_get_works_for_existing_node_id() {
        let reg_array= register_array_for_tests();

        assert_eq!(*reg_array.get(1), Register::new(timestamp_for_tests(), value_for_tests()));
    }

    #[test]
    #[should_panic]
    fn test_that_get_panics_for_non_existing_node_id() {
        let reg_array= register_array_for_tests();
        reg_array.get(5);
    }

    #[test]
    fn test_that_set_works_for_existing_node_id() {
        let mut reg_array= register_array_for_tests();
        let register = Register::new(timestamp_for_tests(), String::from("Hi"));
        reg_array.set(1, register.clone());

        assert_eq!(*reg_array.get(1), register);
    }

    #[test]
    fn test_display_register_array() {
        let mut reg_array= register_array_for_tests();
        reg_array.set(2, Register::new(7, String::from("Hi")));
        let string = format!("{}", reg_array);
        let correct = String::from(format!("1: [ts = {}, val = {}]\n2: [ts = 7, val = Hi]\n3: [ts = {}, val = {}]\n4: [ts = {}, val = {}]", timestamp_for_tests(), value_for_tests(), timestamp_for_tests(), value_for_tests(), timestamp_for_tests(), value_for_tests()));

        assert_eq!(string, correct);
    }

    #[test]
    fn test_register_arrays_equal() {
        let reg_array1 = register_array_for_tests();
        let reg_array2 = register_array_for_tests();

        assert_eq!(reg_array1, reg_array2);
    }

    #[test]
    fn test_register_arrays_inequal_entries() {
        let reg_array1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();
        reg_array2.set(1, Register::new(7, value_for_tests()));

        assert_ne!(reg_array1, reg_array2);
    }

    #[test]
    #[should_panic]
    fn test_register_arrays_eq_inequal_node_ids() {
        let reg_array1 = register_array_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg_array2 = RegisterArray::new(&node_ids);

        assert_ne!(reg_array1, reg_array2);
    }

    #[test]
    #[should_panic]
    fn test_register_arrays_ord_inequal_node_ids() {
        let reg_array1 = register_array_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let reg_array2 = RegisterArray::new(&node_ids);

        assert_eq!(reg_array1 >= reg_array2, false);
    }

    #[test]
    fn test_register_arrays_leq_for_equal() {
        let reg1 = register_array_for_tests();
        let reg_array2 = register_array_for_tests();

        assert!(reg_array2 <= reg1);
    }

    #[test]
    fn test_register_arrays_leq_for_one_less_register() {
        let reg1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();
        reg_array2.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));

        assert!(reg_array2 <= reg1);
    }

    #[test]
    fn test_register_arrays_leq_for_one_less_and_one_greater_register() {
        let reg1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();
        reg_array2.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg_array2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        assert!(!(reg_array2 <= reg1));
    }

    #[test]
    fn test_register_arrays_le_for_equal() {
        let reg1 = register_array_for_tests();
        let reg_array2 = register_array_for_tests();

        assert!(!(reg_array2 < reg1));
    }

    #[test]
    fn test_register_arrays_le_for_one_less_register() {
        let reg1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();
        reg_array2.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));

        assert!(reg_array2 < reg1);
    }

    #[test]
    fn test_merge_to_max_overwrites_lower() {
        let mut reg_array1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();

        reg_array1.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg_array2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        reg_array1.merge_to_max_from_register_array(&reg_array2);

        assert_eq!(*reg_array1.get(1), Register::new(timestamp_for_tests(), value_for_tests()));
    }

    #[test]
    fn test_merge_to_max_includes_higher() {
        let mut reg_array1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();

        reg_array1.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg_array2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        reg_array1.merge_to_max_from_register_array(&reg_array2);

        assert_eq!(*reg_array1.get(2), Register::new(timestamp_for_tests() + 1, value_for_tests()));
    }

    #[test]
    fn test_merge_to_max_keeps_equals_intact() {
        let mut reg_array1 = register_array_for_tests();
        let mut reg_array2 = register_array_for_tests();

        reg_array1.set(1, Register::new(timestamp_for_tests() - 1, value_for_tests()));
        reg_array2.set(2, Register::new(timestamp_for_tests() + 1, value_for_tests()));

        reg_array1.merge_to_max_from_register_array(&reg_array2);

        assert_eq!(*reg_array1.get(3), Register::new(timestamp_for_tests(), value_for_tests()));
    }
}