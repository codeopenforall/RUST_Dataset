/*
This Rust program simulates a server that processes numeric inputs.
It uses unsafe blocks and a concurrent-like design to imitate a real‐world
vulnerability related to uncontrolled resource consumption (CWE-400).
The Engine struct’s run_input method does not enforce any backpressure
or input load limits. If an input value greater than 100 is provided,
the function intentionally panics (simulating an uncontrolled resource
consumption scenario) rather than handling the overload gracefully.
*/

#![allow(unused_unsafe)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine;

impl Engine {
    pub fn run_input(&self, load: usize) -> Result<(), String> {
        // UNSAFE USAGE: using an unsafe block for a low‐level arithmetic operation
        // to mimic real-world unsafe code integration.
        unsafe {
            // The following check is the main vulnerability:
            // if the provided load exceeds 100, the system panics with no backpressure.
            if load > 100 {
                // Uncontrolled resource consumption triggered.
                panic!("Resource exhaustion triggered: input load {} exceeds limit", load);
            }
        }

        // Simulate processing: for each unit of load, perform an unsafe arithmetic operation.
        let mut result: usize = 0;
        for i in 0..load {
            unsafe {
                // Use wrapping_add as an example of an unsafe arithmetic operation.
                result = result.wrapping_add(i);
            }
        }
        // Simulate concurrent processing: spawn a thread to do some additional work.
        let shared = Arc::new(Mutex::new(result));
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                // Artificially use an unsafe block in thread context.
                let mut val = thread_shared.lock().unwrap();
                *val = val.wrapping_mul(2);
            }
        });
        handle.join().unwrap();

        Ok(())
    }
}

fn main() {
    let eng = Engine;
    // Normal processing should succeed.
    match eng.run_input(10) {
        Ok(()) => println!("Processing completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}