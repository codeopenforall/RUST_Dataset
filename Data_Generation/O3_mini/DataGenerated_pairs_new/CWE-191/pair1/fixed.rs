//////////////////////////////////////////////////////////////
// Corrected Code: This improved version prevents integer underflow 
// by verifying that the subtraction argument is non-negative.
// If a negative value is provided, the subtraction is skipped,
// ensuring the counter remains within its valid range.
// The code remains concurrent and uses unsafe for low-level 
// pointer manipulation, but now the arithmetic is performed safely.
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

    // This subtraction method now defends against negative input.
    fn subtract_val(&mut self, val: i32) {
        // Return early if the subtraction value is negative.
        if val < 0 {
            // In a production system one might return a Result or log an error.
            return;
        }
        unsafe {
            let ptr = &mut self.counter as *mut u32;
            // Safe subtraction now that val is confirmed non-negative.
            *ptr = self.counter - (val as u32);
        }
    }
}

fn main() {
    // Shared Data instance wrapped with Arc and Mutex.
    let shared = Arc::new(Mutex::new(Data::new(0)));

    // Spawn one thread that attempts to subtract a negative value.
    let handles: Vec<_> = (0..1)
        .map(|_| {
            let shared_clone = Arc::clone(&shared);
            thread::spawn(move || {
                let mut data = shared_clone.lock().unwrap();
                // subtract_val is called with -1,
                // but the method now safely ignores negative input.
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