/* 
   This corrected version ensures that exceptional conditions (such as parsing errors) are properly handled
   rather than being silently swallowed. The process function now returns a Result, propagating the error to the
   caller. Consequently, the initialization flow aborts if configuration parsing fails, thereby enforcing the necessary
   invariants. Concurrency is maintained without resorting to unsafe pointer dereferencing.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    config: i32,
}

impl Engine {
    // A safe constructor.
    fn new(config: i32) -> Self {
        Engine { config }
    }

    fn run(&self) {
        let shared = Arc::new(Mutex::new(self.config));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let s = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                // Safe access to the shared resource.
                let locked = s.lock().unwrap();
                *locked
            });
            handles.push(handle);
        }
        for handle in handles {
            let res = handle.join();
            if res.is_err() {
                panic!("Thread encountered a problem");
            }
        }
    }
}

// The fix: Properly propagate errors during parsing by using map_err and the try operator.
// An invalid input (e.g. "bad") now yields an Err value.
fn process(input: &str) -> Result<Engine, String> {
    let config_val = input
        .parse::<i32>()
        .map_err(|e| format!("Configuration parse error: {}", e))?;
    Ok(Engine::new(config_val))
}

// A helper API common to external testing.
// The initialization returns an error for invalid configuration values.
pub fn initialize(input: &str) -> Result<i32, String> {
    let engine = process(input)?;
    Ok(engine.config)
}

fn main() {
    match process("bad") {
        Ok(engine) => engine.run(),
        Err(e) => {
            eprintln!("Failed to initialize engine: {}", e);
            std::process::exit(1);
        }
    }
}