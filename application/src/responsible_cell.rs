
use std::cell::UnsafeCell;
use std::mem;

/*
In order to create a circular reference, I need to create
references to both objects, but at the same time create
mutable references to them, so I can modify them to
include the reference to the other object. This is why I
created ResponsibleCell. How the above is achieved is shown
in the Mediator struct.

This violates Rust's borrowing rules even at run-time.
However, I haven't seen a problem doing this. The code
doesn't panic. It might be because this struct is only
used in a "responsible" way, hence the name. Because after
the intial wiring-up of circular references, the mutable
references are dropped and onwards only immutable references
are made. Furthermore, the initialiazation happens on
only a single thread.
*/
pub struct ResponsibleCell<T> {
    unsafe_cell: UnsafeCell<T>
}

unsafe impl<T> Sync for ResponsibleCell<T> {}

impl<T> ResponsibleCell<T> {
    pub fn new(value: T) -> ResponsibleCell<T> {
        ResponsibleCell {
            unsafe_cell: UnsafeCell::new(value)
        }
    }

    pub fn get(&self) -> &T {
        unsafe {
            & *self.unsafe_cell.get()
        }
    }
    
    pub fn get_mut(&self) -> &mut T {
        unsafe {
            &mut *self.unsafe_cell.get()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        pub inner: String
    }

    #[test]
    fn test_double_mutation() {
        let test_value = TestStruct {
            inner: String::from("hej")
        };
        let cell = ResponsibleCell::new(test_value);
        
        let mut1 = cell.get_mut();
        let mut2 = cell.get_mut();

        mut1.inner = String::from("hi");
        mut2.inner = String::from("hello");

        let imut = cell.get();

        assert_eq!(mut1.inner, String::from("hello"));
        assert_eq!(mut2.inner, String::from("hello"));
        assert_eq!(imut.inner, String::from("hello"));
    }
}