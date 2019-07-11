
use std::fmt;
use std::fmt::Display;
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};


pub type Timestamp = i32;

pub fn default_timestamp() -> Timestamp {
    -1
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

    #[test]
    fn test_display() {
        let correct = "[ts = 2, val = Rust]";
        let entry = Entry::new(2, "Rust");

        assert_eq!(format!("{}", entry), correct);
    }

    #[test]
    fn value_does_not_affect_eq() {
        let entry1 = Entry::new(3, "Rust");
        let entry2 = Entry::new(3, "Haskell");

        assert_eq!(entry1, entry2);
    }

    #[test]
    fn entry_eq_is_same_as_timestamp_eq() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(Entry::new(i, "") == Entry::new(j, ""), i == j);
            }
        }
    }

    #[test]
    fn entry_leq_is_same_as_timestamp_leq() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(Entry::new(i, "") <= Entry::new(j, ""), i <= j);
            }
        }
    }

    #[test]
    fn entry_le_is_same_as_timestamp_le() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(Entry::new(i, "") < Entry::new(j, ""), i < j);
            }
        }
    }
}