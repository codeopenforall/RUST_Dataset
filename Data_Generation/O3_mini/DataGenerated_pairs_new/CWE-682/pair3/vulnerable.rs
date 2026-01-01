////////////////////////////////////////////////////////////////////////////////
// Vulnerability Example: Incorrect Calculation using Unsafe Arithmetic
////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Computor {
    multiplier: u32,
}

impl Computor {
    // This method is intended to compute (a * b) / c,
    // but mistakenly computes a * (b / c) due to incorrect grouping.
    // When b < c (as in our test case), b/c yields 0 (integer division),
    // leading to an erroneous result.
    unsafe fn calculate(&self, a: u32, b: u32, c: u32) -> u32 {
        // Vulnerability is on the arithmetic operation below.
        a * (b / c)
    }
}

fn main() {
    // Create shared mutable state with concurrency
    let shared = Arc::new(Mutex::new(10u32));
    let shared_clone = Arc::clone(&shared);

    // Spawn a thread that unsafely updates the shared state.
    let handle = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            // Unsafe raw pointer manipulation of shared data.
            let ptr: *mut u32 = &mut *data;
            *ptr += 5; // arbitrary update
        }
    });

    // Wait for the thread to complete its execution.
    handle.join().unwrap();

    // Initialize the computation structure using the current shared value.
    let comp = Computor { multiplier: *shared.lock().unwrap() };

    // Perform the calculation using an input scenario that triggers the bug:
    // For inputs a = 100, b = 3, c = 10, the intended result is (100 * 3)/10 = 30,
    // but due to the vulnerability, b/c evaluates to 0, so 100 * 0 is 0.
    let result = unsafe { comp.calculate(100, 3, 10) };
    println!("Outcome: {}", result);
}