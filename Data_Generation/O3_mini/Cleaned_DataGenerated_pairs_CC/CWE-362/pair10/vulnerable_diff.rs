use std::cell::UnsafeCell;
use std::sync::Arc;
    counter: UnsafeCell<u32>,
unsafe impl Sync for Shared {}
            counter: UnsafeCell::new(0),
                unsafe {
                    let current = *data.counter.get();
                    if current < 10000 {
                        thread::sleep(Duration::from_micros(1));
                        *data.counter.get() = current + 1;
                    }
    unsafe { *shared.counter.get() }
