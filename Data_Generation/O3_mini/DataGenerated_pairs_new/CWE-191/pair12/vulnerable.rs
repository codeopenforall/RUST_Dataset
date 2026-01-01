/////////////////////////////////////////////////////////////////
// Vulnerability demonstration: Integer underflow via unsafe subtraction.
// This module uses a struct with an unsafe subtraction method that
// does not check for underflow, resulting in wraparound when subtracting
// a larger value from a smaller one. It simulates a real-world vulnerability
// observed in some Rust crates.
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    pub count: u32,
}

impl Data {
    pub fn new(val: u32) -> Self {
        Data { count: val }
    }

    // Unsafe subtraction without underflow check.
    pub unsafe fn reduce(&mut self, sub: u32) -> u32 {
        // Vulnerability: No check is performed. If self.count < sub,
        // this subtraction will wrap around per Rust's wrapping behavior.
        self.count = self.count - sub;  // Vulnerability occurs here.
        self.count
    }
}

// Public API that is used by external callers.
pub fn execute_subtraction() -> Result<u32, &'static str> {
    let mut data = Data::new(5); // Deliberately small value.
    // Unsafe block to call the unchecked reduction.
    let res = unsafe { data.reduce(10) }; // Underflow: 5 - 10 wraps to a large value.
    Ok(res)
}

fn main() {
    let shared = Arc::new(Mutex::new(Data::new(5)));
    let threads: Vec<_> = (0..1)
        .map(|_| {
            let s = Arc::clone(&shared);
            thread::spawn(move || {
                let mut lock = s.lock().unwrap();
                // Unsafe subtraction is performed here.
                unsafe {
                    let r = lock.reduce(10);
                    println!("Subtraction result: {}", r);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}