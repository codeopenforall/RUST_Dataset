//////////////////////////////
// Vulnerable Implementation
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    value: i32,
}

impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }

    // This function performs an arithmetic division using an unsafe block.
    // It does not check whether the divisor is zero.
    // A division by zero will trigger a panic at runtime.
    fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        unsafe {
            // Vulnerability: No check for zero divisor (CWE-369)
            // Lines below implement the unsafe division.
            let result = self.value / divisor;
            Ok(result)
        }
    }
}

fn main() {
    // Simulate concurrent access using Arc and Mutex.
    let engine = Arc::new(Mutex::new(Engine::new(100)));
    let engine_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        let engine = engine_clone.lock().unwrap();
        // This call with divisor=0 will trigger a division by zero panic.
        let _ = engine.exec(0);
    });
    let _ = handle.join();
    println!("Operation completed.");
}