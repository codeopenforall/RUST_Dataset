use std::{cell::UnsafeCell, sync::Arc, thread, time::Duration};
    flag: UnsafeCell<bool>,
    init_count: UnsafeCell<u32>,
unsafe impl Sync for Resource {}
            flag: UnsafeCell::new(false),
            init_count: UnsafeCell::new(0),
        unsafe {
            if !*self.flag.get() {
                thread::sleep(Duration::from_millis(10));
                *self.init_count.get() = *self.init_count.get() + 1;
                *self.flag.get() = true;
        unsafe { *self.init_count.get() }
