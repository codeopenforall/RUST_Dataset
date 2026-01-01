///////////////////////////////////////////////////////////////
// This is the corrected Rust code that fixes the bounds checking
// vulnerability by verifying that the index is within allowed limits.
// The code still uses unsafe pointer arithmetic but only after proper
// validation, eliminating the CWE-119 issue.
///////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

trait Operations {
    fn update(&self, index: usize, value: i32);
}

struct Buffer {
    data: Mutex<Vec<i32>>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}

impl Operations for Buffer {
    fn update(&self, index: usize, value: i32) {
        let mut guard = self.data.lock().unwrap();
        // Check if the index is within bounds before performing any unsafe operations.
        if index < guard.len() {
            unsafe {
                // Using .add() makes the intent clear for pointer arithmetic.
                let ptr = guard.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            // Panic to prevent unsafe out-of-bounds writes.
            panic!("Attempt to write outside of buffer bounds");
        }
    }
}

fn main() {
    let buf = Arc::new(Buffer::new(10));
    
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let b = Arc::clone(&buf);
            thread::spawn(move || {
                // This update will panic because index 15 is invalid.
                // The panic is expected and demonstrates safe behavior.
                b.update(15, 99);
            })
        })
        .collect();

    // Joining threads; if any thread panics, the panic is propagated.
    for h in handles {
        h.join().expect("Thread panicked as expected due to invalid access");
    }

    let guard = buf.data.lock().unwrap();
    println!("Buffer contents: {:?}", *guard);
}