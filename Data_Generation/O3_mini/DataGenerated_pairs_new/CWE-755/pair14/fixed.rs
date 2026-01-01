//////////////////////////////////////////
// Fixed version addressing CWE-755
// Properly propagates errors from exceptional conditions
// by not swallowing errors. It returns a Result type.
//////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct Processor {
    base: i32,
}

impl Processor {
    fn new(base: i32) -> Self {
        Processor { base }
    }

    // This method properly handles errors. It returns an error if the input cannot be parsed
    // or if the divisor is zero, avoiding division by zero and unsafe behavior.
    fn operate(&self, input: &str) -> Result<i32, String> {
        let divisor: i32 = input
            .parse()
            .map_err(|_| "Parsing the divisor failed".to_string())?;
        if divisor == 0 {
            return Err("Divisor cannot be zero".to_string());
        }
        unsafe {
            let ptr = &self.base as *const i32;
            let value = *ptr;
            Ok(value / divisor)
        }
    }
}

// Public API function with improved error propagation.
pub fn perform(input: &str) -> Result<i32, String> {
    let processor = Processor::new(100);
    processor.operate(input)
}

fn main() {
    // Using Arc to share the processor instance across threads.
    let processor = Arc::new(Processor::new(100));
    let arg = std::env::args().nth(1).unwrap_or_default();

    let mut handles = Vec::new();
    for _ in 0..4 {
        let processor = Arc::clone(&processor);
        let arg_clone = arg.clone();
        handles.push(thread::spawn(move || {
            match processor.operate(&arg_clone) {
                Ok(result) => println!("Computed result: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        }));
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}