use std::cell::UnsafeCell;
use std::sync::Arc;
    count: UnsafeCell<i32>,
unsafe impl Sync for SharedData {} 
            count: UnsafeCell::new(val),
        unsafe {
            let old = *self.count.get();
            thread::yield_now(); 
            *self.count.get() = old + 1;
        }
        unsafe { *self.count.get() }
