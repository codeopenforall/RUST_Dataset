///////////////////////////////////////////////
// BEGIN RUST CODE SNIPPET
///////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    threshold: i32,
}

impl Processor {
    fn new(threshold: i32) -> Self {
        Processor { threshold }
    }
    
    fn validate(&self, input: i32) -> bool {
        let mut allowed = false;
        unsafe {
            // Flawed control flow:
            // The unsafe block uses a dummy pointer read and a match statement
            // that incorrectly sets authorization to true regardless of input.
            let dummy = 1;
            let p = &dummy as *const i32;
            let _temp = *p; // simulate a computation
            match input {
                n if n >= self.threshold => allowed = true,
                _ => allowed = true, // BUG: Should be false but always returns true
            }
        }
        allowed
    }
    
    fn process(&self, input: i32) -> Result<String, &'static str> {
        if self.validate(input) {
            Ok(format!("Processed value: {}", input))
        } else {
            Err("Access Denied")
        }
    }
}

pub fn main() {
    // Create a shared instance with a security threshold.
    let proc = Arc::new(Mutex::new(Processor::new(50)));
    let proc_clone = proc.clone();
    
    // Spawn a thread to simulate concurrent processing.
    let handle = thread::spawn(move || {
        let p = proc_clone.lock().unwrap();
        // Input below threshold should be rejected, but due to the flaw it is accepted.
        p.process(20)
    });
    
    let result = handle.join().unwrap();
    match result {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}
///////////////////////////////////////////////
// END RUST CODE SNIPPET