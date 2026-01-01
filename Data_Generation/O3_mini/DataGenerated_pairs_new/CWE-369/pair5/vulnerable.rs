//////////////////////////////
// Vulnerable Version Module
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Processor {
    numerator: i32,
    denominator: i32,
}

impl Processor {
    // Unsafe division without checking for zero.
    unsafe fn operation(&self) -> i32 {
        let numer_ptr: *const i32 = &self.numerator;
        let denom_ptr: *const i32 = &self.denominator;
        // Vulnerability: Division without zero-check.
        *numer_ptr / *denom_ptr
    }
}

// Public API function: performs a division operation concurrently.
pub fn compute(n1: i32, n2: i32) -> Result<i32, &'static str> {
    let proc = Processor {
        numerator: n1,
        denominator: n2,
    };
    let shared = Arc::new(proc);
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        // Unsafe block performing division without validating denominator.
        unsafe { shared_clone.operation() }
    });
    // If denominator is zero, the spawned thread panics; join() then returns Err.
    match handle.join() {
        Ok(res) => Ok(res),
        Err(_) => Err("Unhandled division by zero"),
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