use std::cell::UnsafeCell;
use std::sync::Arc;
    value: UnsafeCell<u32>,
unsafe impl Sync for Data {}
        unsafe {
            let ptr = self.value.get();
            let curr = *ptr;
            thread::yield_now();
            *ptr = curr + 1;
        }
        unsafe { *self.value.get() }
    let shared = Arc::new(Data { value: UnsafeCell::new(0) });
