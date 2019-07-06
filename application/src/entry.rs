
use std::fmt;
use std::fmt::Display;

use std::cmp;
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