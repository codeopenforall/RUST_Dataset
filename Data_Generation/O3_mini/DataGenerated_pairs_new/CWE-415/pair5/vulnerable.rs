use std::sync::atomic::{AtomicBool, Ordering};
use std::panic;

static FREED: AtomicBool = AtomicBool::new(false);

struct Data {
    ptr: *mut i32,
}

impl Data {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        // Vulnerable clone: shallow copy causes two instances to share the same pointer.
        Data { ptr: self.ptr }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        // Faulty drop: use a global flag to detect double free and panic.
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("double free detected");
        }
        unsafe {
            // Manually deallocate the memory.
            let _ = Box::from_raw(self.ptr);
        }
    }
}

fn execute() {
    let original = Data::new(42);
    // Shallow-copying the resource produces two owners of the same pointer.
    let duplicate = original.clone();
    // When both go out of scope, the Drop implementation is called twice,
    // triggering the double free detection.
}

fn main() {
    // Catch any panic to allow graceful termination and diagnostic output.
    let result = panic::catch_unwind(|| {
        execute();
    });
    if result.is_err() {
        eprintln!("Error encountered during execution.");
    }
}