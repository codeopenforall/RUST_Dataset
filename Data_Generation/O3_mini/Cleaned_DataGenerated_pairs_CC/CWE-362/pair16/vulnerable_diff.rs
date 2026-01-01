use std::cell::UnsafeCell;
    flag: UnsafeCell<bool>,
        Self { flag: UnsafeCell::new(true) }
        unsafe {
            if *self.flag.get() {
                thread::sleep(Duration::from_micros(10));
                *self.flag.get() = false;
                return true;
            }
unsafe impl Sync for Core {}
