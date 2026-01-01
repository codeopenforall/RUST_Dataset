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
            // Corrected branch logic:
            // The dummy pointer read is preserved to mimic real-world computation,
            // but the match now properly denies authorization for inputs below the threshold.
            let dummy = 1;
            let p = &dummy as *const i32;
            let _temp = *p;
            match input {
                n if n >= self.threshold => allowed = true,
                _ => allowed = false,
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
        // Inputs below the threshold are now correctly rejected.
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