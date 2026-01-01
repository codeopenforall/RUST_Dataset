use std::cell::UnsafeCell;
    count: UnsafeCell<u32>,
unsafe impl Sync for Shared {}
        Shared { count: UnsafeCell::new(0) }
        unsafe {
            if *self.count.get() == 0 {
                thread::sleep(Duration::from_millis(50));
                *self.count.get() += 1;
            }
        unsafe { *self.count.get() }
