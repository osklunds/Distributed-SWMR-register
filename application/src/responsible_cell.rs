
use std::cell::UnsafeCell;
use std::mem;

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
        let t = TestStruct {
            inner: String::from("hej")
        };
        let cell = ResponsibleCell::new(t);
        
        let x = cell.get_mut();
        let y = cell.get_mut();

        x.inner = String::from("hi");
        y.inner = String::from("hello");

        let z = cell.get();

        assert_eq!(x.inner, String::from("hello"));
        assert_eq!(y.inner, String::from("hello"));
        assert_eq!(z.inner, String::from("hello"));
    }
}