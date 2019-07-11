
use std::cell::UnsafeCell;

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

