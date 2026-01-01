//////////////////////////////
// Vulnerable Code Snapshot  //
// CWE-754: Improper Check for Exceptional Conditions
//////////////////////////////
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

struct Context {
    cancel_flag: Arc<AtomicBool>,
    timeout_flag: Arc<AtomicBool>,
}

impl Context {
    fn new(cancel: bool, timeout: bool) -> Self {
        Context {
            cancel_flag: Arc::new(AtomicBool::new(cancel)),
            timeout_flag: Arc::new(AtomicBool::new(timeout)),
        }
    }

    // In this implementation the exceptional condition "timeout" is ignored.
    // Only the cancellation flag is checked.
    fn execute(&self) -> Result<i32, &'static str> {
        // Improper check: timeout_flag is completely overlooked.
        if !self.cancel_flag.load(Ordering::SeqCst) {
            // Even if a timeout has occurred, operation is allowed to proceed.
            unsafe {
                // Simulate an unsafe operation (raw pointer manipulation).
                // For demonstration, allocate an integer on the heap.
                let raw_ptr = Box::into_raw(Box::new(256));
                let result = *raw_ptr; // Unsafe dereference
                // Free the allocated memory to avoid leaks.
                Box::from_raw(raw_ptr);
                Ok(result)
            }
        } else {
            Err("Operation cancelled")
        }
    }
}

fn main() {
    // Scenario: cancel_flag = false, but timeout_flag = true.
    // With a proper check, the operation should fail due to timeout.
    // However, the vulnerability causes the function to ignore timeout_flag.
    let ctx = Context::new(false, true);
    match ctx.execute() {
        Ok(val) => println!("Success: Result is {}", val),
        Err(err) => println!("Error: {}", err),
    }
}