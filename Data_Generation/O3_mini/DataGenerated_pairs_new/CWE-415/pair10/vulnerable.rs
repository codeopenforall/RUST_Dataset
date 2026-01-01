use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr;

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Handler {
    raw: *mut i32,
}

impl Handler {
    // Allocates an integer on the heap and returns a new Handler.
    unsafe fn create(num: i32) -> Self {
        let boxed = Box::new(num);
        Self {
            raw: Box::into_raw(boxed),
        }
    }

    // This function releases the underlying resource.
    // BUG: It deallocates memory but does not nullify the pointer.
    unsafe fn dispose(&mut self) {
        if !self.raw.is_null() {
            // If the resource has already been freed, trigger a panic.
            if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected in dispose");
            }
            // Manual deallocation using unsafe conversion;
            // since the pointer is not reset, Drop will try to free it again.
            let _ = Box::from_raw(self.raw);
            // BUG: Failing to nullify self.raw allows Drop to free the same resource.
        }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        unsafe {
            if !self.raw.is_null() {
                if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                    panic!("Double free detected in Drop");
                }
                let _ = Box::from_raw(self.raw);
            }
        }
    }
}

fn main() {
    // Unsafe block to allow manual memory management.
    unsafe {
        let mut obj = Handler::create(100);
        obj.dispose(); // Memory freed explicitly.
        // When obj goes out of scope, Drop is invoked, leading to a double free.
    }
    println!("Execution completed (vulnerable path).");
}