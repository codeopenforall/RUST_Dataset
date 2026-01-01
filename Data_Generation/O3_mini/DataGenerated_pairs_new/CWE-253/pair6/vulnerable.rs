use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    state: Mutex<u32>,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            state: Mutex::new(0),
        }
    }
    // This unsafe function mimics a lower-level API that returns a Result.
    // It returns Err(0) when provided with a boundary (value == 0) to indicate failure.
    unsafe fn execute(&self, value: u32) -> Result<u32, u32> {
        // Simulate some unsafe pointer operation.
        if value == 0 {
            // Failure case
            Err(0)
        } else {
            // Success case
            Ok(value)
        }
    }
    // This method is intended to return true only when the underlying unsafe call succeeds.
    // Vulnerability: it incorrectly treats an error result as a success.
    fn run_process(&self, value: u32) -> bool {
        unsafe {
            let result = self.execute(value);
            // INCORRECT: Misinterpreting Err() as a success condition.
            if result.is_err() {
                return true;
            }
            false
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new());
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let eng = engine.clone();
            thread::spawn(move || {
                // For demonstration, input of 0 should be a failure.
                let outcome = eng.run_process(i % 2);
                if outcome {
                    println!("Thread {}: processed (erroneously considered success)", i);
                } else {
                    println!("Thread {}: processed correctly", i);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}