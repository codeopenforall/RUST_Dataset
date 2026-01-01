/*
This Rust program has been corrected to guard against uncontrolled resource consumption (CWE-400).
The Engine struct’s run_input method now enforces a maximum load threshold.
If an input load greater than 100 is provided, it returns an error rather than panicking.
This change prevents the possibility of a denial-of-service attack by rejecting excessive inputs.
*/

#![allow(unused_unsafe)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine;

impl Engine {
    pub fn run_input(&self, load: usize) -> Result<(), String> {
        const MAX_LOAD: usize = 100;
        // Validate input load before entering unsafe operations.
        if load > MAX_LOAD {
            return Err("Input load too high".to_string());
        }

        // UNSAFE USAGE: For demonstration, we still perform an unsafe arithmetic operation.
        let mut result: usize = 0;
        for i in 0..load {
            unsafe {
                result = result.wrapping_add(i);
            }
        }
        // Simulate concurrent processing with proper constraints.
        let shared = Arc::new(Mutex::new(result));
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
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