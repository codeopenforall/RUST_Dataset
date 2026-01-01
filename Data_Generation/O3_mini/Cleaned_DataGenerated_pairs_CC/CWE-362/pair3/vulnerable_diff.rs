use std::cell::UnsafeCell;
use std::sync::Arc;
    value: UnsafeCell<i32>,
unsafe impl Sync for SharedData {}
            value: UnsafeCell::new(0),
        unsafe {
            let current = *self.value.get();
            let next = current + 1;
            *self.value.get() = next;
        }
        unsafe { *self.value.get() }
        panic!("Race condition detected: expected {} but found {}", NUM_THREADS * ITER, result);
