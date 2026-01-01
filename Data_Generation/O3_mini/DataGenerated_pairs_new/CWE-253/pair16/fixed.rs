//////////////////////////////////////////////
// Fixed code sample for CWE-253:
// Correction: The function return value is now correctly interpreted.
//
// The corrected version retains the same structure and concurrency model,
// but the check in operate() now properly treats a zero return from unsafe_op as success.
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
    // FIX: The return value of unsafe_op() is now correctly checked.
    // A zero return indicates success.
    unsafe fn operate(&self, input: u32) -> Result<u32, &'static str> {
        let ret = self.unsafe_op(input);
        if ret == 0 {
            Ok(input.saturating_mul(2))
        } else {
            Err("operation failed")
        }
    }

    // Unsafe low-level operation simulating a C-like API.
    // It signals success by returning 0, and failure by returning 1.
    // In this simulation, even inputs are valid (returns 0 for success),
    // while odd inputs simulate an error condition.
    unsafe fn unsafe_op(&self, input: u32) -> i32 {
        if input % 2 == 0 { 0 } else { 1 }
    }
}

fn main() {
    let handler = Handler::new();
    let shared = Arc::new(handler);
    let mut threads = vec![];

    // Spawn several threads to mimic realistic concurrent processing.
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