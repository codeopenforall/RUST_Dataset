/* 
   This code emulates a real‐world vulnerability where an exceptional condition – a parsing error – is improperly handled. 
   Instead of propagating the error when converting a configuration value from a string to an integer, the code silently 
   defaults to 0 via unwrap_or_default(). This behavior (swallowing the error) may lead to incorrect initialization, 
   resulting in unintended behavior. 
   The code also uses unsafe blocks and concurrency to mimic realistic usage patterns.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    config: i32,
}

impl Engine {
    // Using an unsafe constructor to simulate low-level initialization.
    unsafe fn new(config: i32) -> Self {
        Engine { config }
    }

    fn run(&self) {
        // Spawn multiple threads to simulate concurrent operations.
        let shared = Arc::new(Mutex::new(self.config));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let s = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                // Deliberately use unsafe to dereference a raw pointer.
                unsafe {
                    let locked = s.lock().unwrap();
                    let ptr = (&*locked) as *const i32;
                    // Read the value via the raw pointer.
                    *ptr
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join().unwrap();
        }
    }
}

// The vulnerability: An error during parsing is swallowed by unwrap_or_default,
// resulting in a default value of 0 instead of propagating the error.
fn process(input: &str) -> Engine {
    let config_val = input.parse::<i32>().unwrap_or_default(); // Vulnerable line: error is lost
    unsafe { Engine::new(config_val) }
}

// A helper API common to both versions for external testing.
// In the vulnerable code, an invalid input ("bad") results in a default config of 0.
pub fn initialize(input: &str) -> Result<i32, String> {
    let engine = process(input);
    Ok(engine.config)
}

fn main() {
    // For demonstration, an intentionally invalid input is passed.
    let engine = process("bad");
    engine.run();
}