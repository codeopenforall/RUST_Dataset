#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Struct representing the result of an asynchronous operation.
struct OperationResult {
    is_timeout: bool,
    is_cancelled: bool,
    value: i32,
}

// A trait defining a processing contract.
trait Processor {
    fn process(&self) -> Result<i32, &'static str>;
}

// A worker struct using an internal value.
struct Worker {
    data: i32,
}

impl Processor for Worker {
    fn process(&self) -> Result<i32, &'static str> {
        // Simulate an operation that in our test always signals a timeout.
        let op = simulate_operation();
        // Copy the returned value for local processing.
        let mut local = op.value;
        // Create a mutable raw pointer to the local value.
        let raw_ptr: *mut i32 = &mut local;

        // VULNERABILITY: Instead of checking if a timeout occurred (is_timeout), the code checks the wrong flag (is_cancelled).
        if op.is_cancelled { // ERROR: Should check op.is_timeout.
            unsafe {
                // Unsafe modification triggered by the wrong flag.
                *raw_ptr = -1;
            }
            return Err("Timeout occurred");
        }

        // Proceed with concurrent processing.
        let shared = Arc::new(Mutex::new(local));
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut val = shared_clone.lock().unwrap();
            *val += 1;
        });
        handle.join().unwrap();

        let final_val = *shared.lock().unwrap();
        Ok(final_val)
    }
}

// Simulates an operation that should signal a timeout.
fn simulate_operation() -> OperationResult {
    // In a real scenario, the flags would be set based on actual exceptional conditions.
    OperationResult { is_timeout: true, is_cancelled: false, value: 42 }
}

fn main() {
    let worker = Worker { data: 10 };
    match worker.process() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}