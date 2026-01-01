use std::cell::UnsafeCell;
static mut GLOBAL: UnsafeCell<u32> = UnsafeCell::new(0);
        unsafe {
            let ptr = GLOBAL.get();
            let current = *ptr;
            thread::yield_now();
            *ptr = current.wrapping_add(1);
        }
    unsafe { *GLOBAL.get() }
