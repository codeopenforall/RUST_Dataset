//////////////////////////////////////////
// Vulnerable version targeting CWE-755
// Improper Handling of Exceptional Conditions:
// Swallowing errors via unwrap_or_default.
// This version uses unsafe blocks and concurrency.
//////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct Engine {
    base: i32,
}

impl Engine {
    fn new(base: i32) -> Self {
        Engine { base }
    }

    // This method improperly swallows errors by using unwrap_or_default(),
    // so if the parsing of the input fails, it returns 0 and then performs
    // a division which may cause a panic (division by zero).
    fn process(&self, input: &str) -> i32 {
        // Vulnerability: Swallow parsing error.
        let divisor: i32 = input.parse().unwrap_or_default();  // <-- Vulnerable line
        unsafe {
            // Using an unsafe block to mimic real-world low-level operations.
            let ptr = &self.base as *const i32;
            let value = *ptr;
            // A division by zero (if divisor==0) will panic.
            value / divisor
        }
    }
}

// Public API function to unify the interface.
// In the vulnerable version, any error goes unnoticed; instead,
 // the function returns an Ok() with the computed value. Panics may occur.
pub fn perform(input: &str) -> Result<i32, String> {
    let engine = Engine::new(100);
    // Wrap the call in catch_unwind when used in testing.
    Ok(engine.process(input))
}

fn main() {
    // Using Arc to share the engine across threads.
    let engine = Arc::new(Engine::new(100));
    // Accept input from command-line arguments.
    let arg = std::env::args().nth(1).unwrap_or_default();

    // Spawn several threads to simulate concurrent operations.
    let mut handles = Vec::new();
    for _ in 0..4 {
        let engine = Arc::clone(&engine);
        let arg_clone = arg.clone();
        handles.push(thread::spawn(move || {
            // This call might panic if input is invalid (e.g. non-numeric).
            let res = engine.process(&arg_clone);
            println!("Computed result: {}", res);
        }));
    }
    for handle in handles {
        // Join all threads; if any panics, the process will be aborted.
        handle.join().expect("Thread panicked");
    }
}