use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Processor {
    threshold: i32,
}

impl Processor {
    fn new() -> Self {
        Processor { threshold: 0 }
    }

    // This method performs a computation that may “fail” for negative inputs.
    // An unsafe block is used to simulate a complex operation.
    // If the input is negative, a Result error is returned.
    // However, the error is immediately swallowed by unwrap_or_default,
    // which causes the function to silently return the default value 0.
    fn process(&self, input: i32) -> i32 {
        unsafe {
            let res: Result<i32, &'static str> = if input < 0 {
                Err("Negative value not allowed")
            } else {
                Ok(input + 42)
            };
            // Vulnerability: The unwrap_or_default call swallows the error,
            // returning 0 instead of propagating the failure.
            res.unwrap_or_default()
        }
    }
}

fn main() {
    let proc = Arc::new(Mutex::new(Processor::new()));
    let mut handles = Vec::new();
    
    let inputs = vec![10, -5, 20, -1, 0];
    
    for &inp in &inputs {
        let proc_clone = Arc::clone(&proc);
        let handle = thread::spawn(move || {
            let p = proc_clone.lock().unwrap();
            // Negative inputs lead to a default value of 0, hiding the error.
            let result = p.process(inp);
            println!("Input {} => Output {}", inp, result);
        });
        handles.push(handle);
    }
    
    for h in handles {
        h.join().unwrap();
    }
}