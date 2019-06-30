
use std::collections::{HashMap, HashSet};

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;

use std::cmp::Ordering;

pub struct Register<V> {
    map: HashMap<i32, Entry<V>>
}

impl<V: Default> Register<V> {
    pub fn new(node_ids: HashSet<i32>) -> Register<V> {
        let mut map = HashMap::new();
        for node_id in node_ids {
            map.insert(node_id, Entry::new(-1, V::default()));
        }

        Register {
            map: map
        }
    }

    pub fn get(self: &Self, node_id: i32) -> Option<&Entry<V>> {
        self.map.get(&node_id)
    }

    pub fn set(self: &mut Self, node_id: i32, entry: Entry<V>) {
        self.map.insert(node_id, entry);
    }
}

impl<V: Display> Display for Register<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for node_id in self.map.keys() {
            let entry = &self.map.get(node_id).unwrap();
            string.push_str(&format!("{}: {}", node_id, entry));
        }

        write!(f, "{}", string)
    }
}

impl<V: PartialEq> PartialEq for Register<V> {
    fn eq(&self, other: &Self) -> bool {
        for node_id in self.map.keys() {
            let my_val = self.map.get(&node_id);
            let other_val = other.map.get(&node_id);

            if my_val != other_val {
                return false;
            }
        }
        return true;
    }
}

impl<V: PartialOrd> PartialOrd for Register<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if less_than_or_equal(&self.map, &other.map) {
            Some(Ordering::Less)
        } else if less_than_or_equal(&other.map, &self.map) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

fn less_than_or_equal<V: PartialOrd>(lhs: &HashMap<i32, V>, rhs: &HashMap<i32, V>) -> bool {
    for node_id in lhs.keys() {
        let lhs_val = lhs.get(&node_id).unwrap();
        let rhs_val = rhs.get(&node_id).expect("Attempting to compare registers with different keys.");

        if lhs_val > rhs_val {
            return false;
        }
    }
    return true;
}

pub struct Entry<V> {
    pub ts: i32,
    pub val: V
}

impl<V> Entry<V> {
    fn new(ts: i32, val: V) -> Entry<V> {
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

impl<V> PartialOrd for Entry<V> {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        self.ts.partial_cmp(&other.ts)
    }
}
