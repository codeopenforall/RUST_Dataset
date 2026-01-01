use std::cell::UnsafeCell;
use std::sync::Arc;
use std::time::Duration;
    value: UnsafeCell<u32>,
unsafe impl Sync for Data {}
            value: UnsafeCell::new(val),
        unsafe {
            let ptr = self.value.get();
            let current = ptr.read(); 
            thread::sleep(Duration::from_micros(1));
            ptr.write(current + 1); 
        }
        unsafe { self.value.get().read() }
