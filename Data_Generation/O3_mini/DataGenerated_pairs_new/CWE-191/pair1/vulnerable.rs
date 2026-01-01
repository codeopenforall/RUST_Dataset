//////////////////////////////////////////////////////////////
// Vulnerable Code: This example simulates a scenario where an
// unsigned counter is manipulated concurrently. An unsafe
// subtraction method subtracts an i32 value from the counter by
// directly casting it to u32 without verifying its sign.
// This creates an integer underflow vulnerability (CWE-191)
// when a negative value is provided.
//////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    counter: u32,
}

impl Data {
    fn new(count: u32) -> Self {
        Data { counter: count }
    }

    // This subtraction method is vulnerable.
    // Converting a negative i32 to u32 produces a very large value,
    // causing an unintended wraparound.
    fn subtract_val(&mut self, val: i32) {
        unsafe {
            // Vulnerable: negative val is cast to u32 and subtracted.
            let ptr = &mut self.counter as *mut u32;
            *ptr = self.counter - (val as u32);
        }
    }
}

fn main() {
    // Create a shared Data instance wrapped in Arc and Mutex.
    let shared = Arc::new(Mutex::new(Data::new(0)));

    // Spawn one thread that invokes the subtraction with a negative input.
    let handles: Vec<_> = (0..1)
        .map(|_| {
            let shared_clone = Arc::clone(&shared);
            thread::spawn(move || {
                let mut data = shared_clone.lock().unwrap();
                // Here subtract_val is called with -1,
                // which when cast to u32 becomes 4294967295.
                data.subtract_val(-1);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let data = shared.lock().unwrap();
    println!("Final counter value: {}", data.counter);
}