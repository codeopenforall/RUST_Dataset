//////////////////////////////
// Fixed Version Module
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Processor {
    numerator: i32,
    denominator: i32,
}

impl Processor {
    // Safe division with an explicit zero-check.
    fn operation(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Division by zero error");
        }
        // Although the computation is simple, we mimic usage of unsafe code.
        unsafe {
            let numer_ptr: *const i32 = &self.numerator;
            let denom_ptr: *const i32 = &self.denominator;
            Ok(*numer_ptr / *denom_ptr)
        }
    }
}

// Public API function that performs the division concurrently.
pub fn compute(n1: i32, n2: i32) -> Result<i32, &'static str> {
    let proc = Processor {
        numerator: n1,
        denominator: n2,
    };
    let shared = Arc::new(proc);
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        // Call the safe operation that checks for zero.
        shared_clone.operation()
    });
    // Flatten the result from the thread join.
    match handle.join() {
        Ok(inner) => inner,
        Err(_) => Err("Thread error"),
    }
}

fn main() {
    // Read the denominator from command-line argument; defaults to "0" if missing.
    let arg = std::env::args().nth(1).unwrap_or_else(|| "0".to_string());
    let denom = arg.parse::<i32>().unwrap_or(0);
    match compute(100, denom) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}