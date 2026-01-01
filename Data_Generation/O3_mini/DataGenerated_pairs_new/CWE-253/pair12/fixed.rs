//////////////////////////////////////////////
// Fixed Code Sample - CWE-253 Correction
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Processor {}

impl Processor {
    // Performs a basic calculation; returns an error if input is invalid.
    // Unsafe block simulates low-level operations.
    unsafe fn compute(&self, input: i32) -> Result<i32, &'static str> {
        // Simulated unsafe pointer manipulation (for demonstration)
        let ptr = &input as *const i32;
        let val = *ptr;
        if val < 0 {
            return Err("Negative input not allowed");
        }
        Ok(val * 2)
    }

    // Correctly propagates the error from the unsafe computation.
    pub fn process(&self, input: i32) -> Result<i32, &'static str> {
        unsafe {
            let outcome = self.compute(input);
            // Fixed: Do not mask the error; propagate it to the caller.
            outcome
        }
    }
}

fn main() {
    let processor = Arc::new(Processor {});
    let results = Arc::new(Mutex::new(Vec::new()));

    // Spawn multiple threads with a mix of valid and boundary (invalid) inputs.
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let proc_clone = Arc::clone(&processor);
            let res_clone = Arc::clone(&results);
            thread::spawn(move || {
                // Use positive input for even threads, and negative input for odd threads.
                let input = if i % 2 == 0 { 10 } else { -5 };
                let outcome = proc_clone.process(input);
                let mut vec = res_clone.lock().unwrap();
                // Append the result. For errors, we record -1 to indicate failure.
                vec.push(match outcome {
                    Ok(val) => val,
                    Err(_) => -1,
                });
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let res = results.lock().unwrap();
    println!("Processing outputs: {:?}", *res);
    // In the fixed code, negative input (-5) correctly results in an error, represented here as -1.
}