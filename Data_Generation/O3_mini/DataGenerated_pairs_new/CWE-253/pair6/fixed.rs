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
    // The same unsafe function simulating a lower-level API operation.
    unsafe fn execute(&self, value: u32) -> Result<u32, u32> {
        if value == 0 {
            Err(0)
        } else {
            Ok(value)
        }
    }
    // Corrected method: now correctly returns true only when the call succeeds.
    fn run_process(&self, value: u32) -> bool {
        unsafe {
            let result = self.execute(value);
            // CORRECT: success is only when result is Ok.
            if result.is_ok() {
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
                let outcome = eng.run_process(i % 2);
                if outcome {
                    println!("Thread {}: processed (success)", i);
                } else {
                    println!("Thread {}: processed (failure)", i);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}