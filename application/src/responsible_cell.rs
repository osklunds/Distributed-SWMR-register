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
    unsafe_cell: UnsafeCell<T>,
}

unsafe impl<T: Sync> Sync for ResponsibleCell<T> {}

impl<T> ResponsibleCell<T> {
    pub fn new(value: T) -> ResponsibleCell<T> {
        ResponsibleCell {
            unsafe_cell: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.unsafe_cell.get() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.unsafe_cell.get() }
    }
}