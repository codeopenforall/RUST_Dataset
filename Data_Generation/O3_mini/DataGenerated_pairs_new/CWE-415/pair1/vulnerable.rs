//////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust program demonstrates a double free vulnerability by duplicating ownership of a raw pointer.
// It simulates a scenario where two instances inadvertently own the same heap allocation.
// In the Drop implementation, a global atomic flag is used to decide which instance frees the memory.
// However, upon the second drop the same pointer is freed again, triggering a double free error.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::atomic::{AtomicBool, Ordering};

static FREED: AtomicBool = AtomicBool::new(false);

struct Data {
    ptr: *mut i32,
}

impl Data {
    fn new(val: i32) -> Self {
        // Allocate memory on the heap and obtain a raw pointer.
        let boxed = Box::new(val);
        Data { ptr: Box::into_raw(boxed) }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            // If memory has not been freed, free it and mark as freed.
            if !FREED.compare_and_swap(false, true, Ordering::SeqCst) {
                let _ = Box::from_raw(self.ptr);
            } else {
                // Erroneously freeing the same memory twice.
                let _ = Box::from_raw(self.ptr);
                // Trigger an error to signal the double free.
                panic!("Double free detected");
            }
        }
    }
}

// The core function that simulates the vulnerable behavior.
fn run_app() -> i32 {
    // Create a new instance managing the heap allocation.
    let instance1 = Data::new(100);
    // UNSAFE: Duplicate the raw pointer, creating a second owner that will attempt to free the memory.
    let _instance2 = Data { ptr: instance1.ptr };
    // Return a sentinel value.
    100
}

fn main() {
    // Execute the application logic.
    let _ = run_app();
}