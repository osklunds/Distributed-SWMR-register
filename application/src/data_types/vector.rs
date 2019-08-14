
use std::collections::{HashMap, HashSet, BTreeMap};
use std::collections::hash_map::IntoIter;
use std::fmt::{Formatter, Display, Result};
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

use commons::types::NodeId;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector<V> {
    map: HashMap<NodeId, V>,
    node_ids: HashSet<NodeId>
}

impl<V: Default + Clone + PartialEq + Eq + PartialOrd> Vector<V> {
    pub fn new(node_ids: &HashSet<NodeId>) -> Vector<V> {
        let mut map = HashMap::new();
        for &node_id in node_ids {
            map.insert(node_id, V::default());
        }

        Vector {
            map: map,
            node_ids: node_ids.clone()
        }
    }

    pub fn get(&self, node_id: NodeId) -> &V {
        self.map.get(&node_id).expect("Trying to get value in Vector, but that node id does not exist.")
    }

    pub fn set(&mut self, node_id: NodeId, value: V) {
        if self.map.insert(node_id, value) == None {
            panic!("Trying to set value in Vector, but that node id does not exist.");
        } 
    }

    pub fn merge_to_max_from_vector(&mut self, other: &Vector<V>) {
        for (node_id, value) in self.map.iter_mut() {
            let other_value = other.map.get(node_id).unwrap();
            if other_value > value {
                *value = other_value.clone(); // Potential future improvement: take ownership of other so that no cloning is needed
            }
        }
    }

    #[allow(dead_code)]
    pub fn node_ids(&self) -> &HashSet<NodeId> {
        &self.node_ids
    }
}

impl<V> Vector<V> {
    fn panic_if_not_same_node_ids(&self, other: &Vector<V>) {
        for node_id in self.map.keys() {
            other.map.get(node_id).expect("Comparing two Vectors with different node ids.");
        }

        for node_id in other.map.keys() {
            self.map.get(node_id).expect("Comparing two Vectors with different node ids.");
        }
    }
}

impl<V: Display> Display for Vector<V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut sorted_map = BTreeMap::new();
        for (node_id, value) in self.map.iter() {
            sorted_map.insert(node_id, value);
        }
        let mut string = String::new();
        for (node_id, value) in sorted_map.iter() {
            string.push_str(&format!("{}: {}\n", node_id, value));
        }

        write!(f, "{}", string.trim_end())
    }
}

impl<V: PartialEq> PartialEq for Vector<V> {
    fn eq(&self, other: &Self) -> bool {
        if cfg!(debug_assertions) {
            self.panic_if_not_same_node_ids(other);
        }

        self.map == other.map
    }
}

impl<V: PartialEq + PartialOrd> PartialOrd for Vector<V> {
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

impl<V> IntoIterator for Vector<V> {
    type Item = (NodeId, V);
    type IntoIter = IntoIter<NodeId, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use commons::types::Int;

    fn node_ids_for_tests() -> HashSet<NodeId> {
        let mut node_ids = HashSet::new();
        node_ids.insert(1);
        node_ids.insert(2);
        node_ids.insert(3);
        node_ids.insert(4);
        node_ids
    }

    fn value_for_tests() -> Int {
        10
    }

    fn vector_for_tests() -> Vector<Int> {
        let mut vec = Vector::new(&node_ids_for_tests());
        for &node_id in node_ids_for_tests().iter() {
            vec.set(node_id, value_for_tests());
        }

        vec
    }

    #[test]
    fn test_that_new_contains_provided_node_ids() {
        let vec = vector_for_tests();

        for node_id in node_ids_for_tests().iter() {
            assert!(vec.map.contains_key(node_id));
        }
    }

    #[test]
    fn test_that_new_contains_no_other_node_ids() {
        let vec = vector_for_tests();

        for node_id in vec.map.keys() {
            assert!(node_ids_for_tests().contains(node_id));
        }
    }

    #[test]
    fn test_that_get_works_for_existing_node_id() {
        let vec = vector_for_tests();

        assert_eq!(*vec.get(1), value_for_tests());
    }

    #[test]
    #[should_panic]
    fn test_that_get_panics_for_non_existing_node_id() {
        let vec = vector_for_tests();
        vec.get(5);
    }

    #[test]
    fn test_that_set_works_for_existing_node_id() {
        let mut vec = vector_for_tests();
        vec.set(1, 7);

        assert_eq!(*vec.get(1), 7);
    }

    #[test]
    fn test_display() {
        let mut vec = vector_for_tests();
        vec.set(2, 7);
        let string = format!("{}", vec);
        let correct = String::from(format!("1: {}\n2: 7\n3: {}\n4: {}", value_for_tests(), value_for_tests(), value_for_tests()));

        assert_eq!(string, correct);
    }

    #[test]
    fn test_vectors_equal() {
        let vec1 = vector_for_tests();
        let vec2 = vector_for_tests();

        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_vectors_inequal_entries() {
        let vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();
        vec2.set(1, 7);

        assert_ne!(vec1, vec2);
    }

    #[test]
    #[should_panic]
    fn test_vectors_eq_inequal_node_ids() {
        let vec1 = vector_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let vec2 = Vector::new(&node_ids);

        assert_ne!(vec1, vec2);
    }

    #[test]
    #[should_panic]
    fn test_vectors_ord_inequal_node_ids() {
        let vec1 = vector_for_tests();
        let mut node_ids = HashSet::new();
        node_ids.insert(5);
        let vec2 = Vector::new(&node_ids);

        assert_eq!(vec1 >= vec2, false);
    }

    #[test]
    fn test_vectors_leq_for_equal() {
        let vec1 = vector_for_tests();
        let vec2 = vector_for_tests();

        assert!(vec2 <= vec1);
    }

    #[test]
    fn test_vectors_leq_for_one_less_value() {
        let vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();
        vec2.set(1, value_for_tests() - 1);

        assert!(vec2 <= vec1);
    }

    #[test]
    fn test_vectors_leq_for_one_less_and_one_greater_value() {
        let vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();
        vec2.set(1, value_for_tests() - 1);
        vec2.set(2, value_for_tests() + 1);

        assert!(!(vec2 <= vec1));
    }

    #[test]
    fn test_vectors_le_for_equal() {
        let vec1 = vector_for_tests();
        let vec2 = vector_for_tests();

        assert!(!(vec2 < vec1));
    }

    #[test]
    fn test_vectors_le_for_one_less_value() {
        let vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();
        vec2.set(1,value_for_tests() - 1);

        assert!(vec2 < vec1);
    }

    #[test]
    fn test_merge_to_max_overwrites_lower() {
        let mut vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();

        vec1.set(1, value_for_tests() - 1);
        vec2.set(2, value_for_tests() + 1);

        vec1.merge_to_max_from_vector(&vec2);

        assert_eq!(*vec1.get(1), value_for_tests());
    }

    #[test]
    fn test_merge_to_max_includes_higher() {
        let mut vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();

        vec1.set(1, value_for_tests() - 1);
        vec2.set(2, value_for_tests() + 1);

        vec1.merge_to_max_from_vector(&vec2);

        assert_eq!(*vec1.get(2), value_for_tests() + 1);
    }

    #[test]
    fn test_merge_to_max_keeps_equals_intact() {
        let mut vec1 = vector_for_tests();
        let mut vec2 = vector_for_tests();

        vec1.set(1, value_for_tests() - 1);
        vec2.set(2, value_for_tests() + 1);

        vec1.merge_to_max_from_vector(&vec2);

        assert_eq!(*vec1.get(3), value_for_tests());
    }
}