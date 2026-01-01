//////////////////////////////////////////////
// Vulnerable code sample for CWE-253:
// Incorrect check of a function’s return value in an unsafe, concurrent context.
//
// This example defines a concurrent processor that calls an unsafe internal function.
// The unsafe operation is intended to signal success with a zero return value and failure with non‐zero.
// However, the check is inverted: non‑zero is treated as success and zero as failure.
// As a result, valid inputs (e.g. even numbers) are rejected while error conditions (e.g. odd numbers) are treated as success.
//////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

pub struct Handler {
    // Shared data for aggregation
    data: Arc<Mutex<Vec<u32>>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Public unsafe function that executes the operation.
    // It is meant to process an input and signal success by returning Ok(result)
    // on success and Err(&str) on failure.
    pub unsafe fn run(&self, input: u32) -> Result<u32, &'static str> {
        self.operate(input)
    }

    // This internal function calls an unsafe operation.
    // Vulnerability: Incorrect check of the unsafe operation's return value.
    // The unsafe_op() returns 0 to indicate success, but the code incorrectly treats any non‑zero
    // as success.
    unsafe fn operate(&self, input: u32) -> Result<u32, &'static str> {
        let ret = self.unsafe_op(input);
        // INCORRECT CHECK: should be "if ret == 0" instead of "if ret != 0"
        if ret != 0 {
            Ok(input.saturating_mul(2))
        } else {
            Err("operation failed")
        }
    }

    // Unsafe low-level operation simulating a C-like API.
    // It signals success by returning 0, and failure by returning 1.
    // In this simulation, even inputs are valid (success) and odd inputs cause an error.
    unsafe fn unsafe_op(&self, input: u32) -> i32 {
        if input % 2 == 0 { 0 } else { 1 }
    }
}

fn main() {
    let handler = Handler::new();
    let shared = Arc::new(handler);
    let mut threads = vec![];

    // Spawn a few threads to mimic concurrent processing.
    for i in 1..=4 {
        let proc = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            unsafe {
                match proc.run(i) {
                    Ok(val) => {
                        let mut agg = proc.data.lock().unwrap();
                        agg.push(val);
                    },
                    Err(e) => eprintln!("Thread input {} error: {}", i, e),
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result = shared.data.lock().unwrap();
    println!("Aggregated result: {:?}", *result);
}