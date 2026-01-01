////////////////////////////////////////////////////////////////////////////////
// Corrected Example: Accurate Calculation with Proper Operator Grouping
////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Computor {
    multiplier: u32,
}

impl Computor {
    // Correctly computes (a * b) / c by ensuring proper operator grouping.
    unsafe fn calculate(&self, a: u32, b: u32, c: u32) -> u32 {
        // Fixed arithmetic: multiplication is performed before division.
        (a * b) / c
    }
}

fn main() {
    // Create shared mutable state with concurrency.
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

    // Wait for the thread to finish.
    handle.join().unwrap();

    // Initialize the computation structure.
    let comp = Computor { multiplier: *shared.lock().unwrap() };

    // For inputs a = 100, b = 3, c = 10, the correct result is (100 * 3)/10 = 30.
    let result = unsafe { comp.calculate(100, 3, 10) };
    println!("Outcome: {}", result);
}