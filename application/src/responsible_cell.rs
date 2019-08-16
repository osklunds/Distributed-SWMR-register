
use std::cell::UnsafeCell;

/*
Interior mutability is needed when Mediator is created. RefCell would
do, but it's not thread-safe. Mutex and RwLock are, but they have a
performance overhead. This responsible cell is thread safe and doesn't
have a performance overhead, but, it must be used responsibly.
That means that mutable references may only be taken before any threading
occurs. Once threaded, no mutable references may appear, and only
immutable once may be used. Which is sufficient for my use of it
in Mediator.
*/
pub struct ResponsibleCell<T> {
    unsafe_cell: UnsafeCell<T>
}

unsafe impl<T: Sync> Sync for ResponsibleCell<T> {}

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