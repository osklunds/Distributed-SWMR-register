
use std::fmt;
use std::fmt::Display;
use std::cmp::Ordering;
use std::default::Default;

use serde::{Serialize, Deserialize};

use commons::types::Int;


pub type Timestamp = Int;

pub fn default_timestamp() -> Timestamp {
    -1
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Register<V> {
    pub ts: Timestamp,
    pub val: V
}

impl<V> Register<V> {
    pub fn new(ts: Timestamp, val: V) -> Register<V> {
        Register {
            ts: ts,
            val: val
        }
    }
}

impl<V: Display> Display for Register<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ts = {}, val = {}]", self.ts, self.val)
    }
}

impl<V> PartialEq for Register<V> {
    fn eq(&self, other: &Self) -> bool {
        self.ts == other.ts
    }
}

impl<V> Eq for Register<V> {}

impl<V> PartialOrd for Register<V> {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        self.ts.partial_cmp(&other.ts)
    }
}

impl<V> Ord for Register<V> {
    fn cmp(&self, other:&Self) -> Ordering {
        self.ts.cmp(&other.ts)
    }
}

impl<V: Default> Default for Register<V> {
    fn default() -> Self {
        Register::new(default_timestamp(), V::default())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let correct = "[ts = 2, val = Rust]";
        let register = Register::new(2, "Rust");

        assert_eq!(format!("{}", register), correct);
    }

    #[test]
    fn value_does_not_affect_eq() {
        let register1 = Register::new(3, "Rust");
        let register2 = Register::new(3, "Haskell");

        assert_eq!(register1, register2);
    }

    #[test]
    fn register_eq_is_same_as_timestamp_eq() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(Register::new(i, "") == Register::new(j, ""), i == j);
            }
        }
    }

    #[test]
    fn register_leq_is_same_as_timestamp_leq() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(Register::new(i, "") <= Register::new(j, ""), i <= j);
            }
        }
    }

    #[test]
    fn register_le_is_same_as_timestamp_le() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(Register::new(i, "") < Register::new(j, ""), i < j);
            }
        }
    }
}