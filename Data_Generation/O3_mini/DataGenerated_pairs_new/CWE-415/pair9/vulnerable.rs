////////////////////////////////////////////////////////////////
// Vulnerability example demonstrating a double release (CWE-415)
use std::sync::atomic::{AtomicBool, Ordering};

static FREED: AtomicBool = AtomicBool::new(false);

struct Item {
    // Holds a raw pointer to an integer allocated on the heap.
    data: *mut i32,
}

impl Item {
    // Allocates a new Item with the integer value 100.
    fn new() -> Self {
        let boxed = Box::new(100);
        Item { data: Box::into_raw(boxed) }
    }
    // Returns the integer value stored in the allocated memory.
    fn value(&self) -> i32 {
        unsafe { *self.data }
    }
}

impl Clone for Item {
    // Vulnerable clone: Performs a shallow copy of the raw pointer.
    // Both the original and the clone will point to the same heap allocation.
    fn clone(&self) -> Self {
        Item { data: self.data }
    }
}

impl Drop for Item {
    // On drop, the heap allocation is freed.
    // If dropped twice (because of cloning) the second drop panics deliberately.
    fn drop(&mut self) {
        unsafe {
            if FREED.load(Ordering::SeqCst) {
                panic!("Double free detected");
            } else {
                FREED.store(true, Ordering::SeqCst);
                // Manually free the memory.
                drop(Box::from_raw(self.data));
            }
        }
    }
}

// Function that runs the application logic.
fn run_app() {
    // Reset the free flag before usage.
    FREED.store(false, Ordering::SeqCst);
    let first = Item::new();
    let second = first.clone();
    // Both items share the same pointer. When they go out of scope,
    // the drop() method is invoked twice, triggering a double free.
    println!("Values: {} and {}", first.value(), second.value());
}

fn main() {
    run_app();
}