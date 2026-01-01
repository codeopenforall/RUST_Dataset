use std::cell::UnsafeCell;
use std::sync::Arc;
    data: UnsafeCell<Vec<i32>>,
unsafe impl Sync for Pool {}
            data: UnsafeCell::new(Vec::new()),
        unsafe {
            (*self.data.get()).push(value);
        unsafe { (*self.data.get()).len() }
    count <= 1000
