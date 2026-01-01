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

    // This function correctly releases the underlying resource.
    // FIX: After deallocation, the pointer is set to null so that Drop does not free it again.
    unsafe fn dispose(&mut self) {
        if !self.raw.is_null() {
            if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected in dispose");
            }
            let _ = Box::from_raw(self.raw);
            self.raw = ptr::null_mut(); // Prevent double free by nullifying the pointer.
        }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        unsafe {
            // Only attempt deallocation if the pointer is non-null.
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
    unsafe {
        let mut obj = Handler::create(200);
        obj.dispose(); // Correctly frees the memory and prevents double free.
    }
    println!("Execution completed (corrected path).");
}