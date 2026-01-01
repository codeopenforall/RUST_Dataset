//////////////////////////////////////////////////////////////
// Vulnerable Code: Incorrect check of a function’s return value
// This program defines a shared data buffer and attempts to update
// a given index concurrently. It uses unsafe blocks to write into
// the buffer and spawns a thread. However, when an out‐of‐bound index
// is used, the update function returns an error—but the code
// misinterprets the error as a successful update (inverted logic),
// causing an incorrect outcome.
//////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    data: Box<[u32]>,
}

impl DataBuffer {
    fn new(size: usize) -> Self {
        let vec = vec![0; size].into_boxed_slice();
        DataBuffer { data: vec }
    }

    // Update the buffer at the given index.
    // Returns Err if the index is out of bounds.
    fn modify(&mut self, index: usize, new_val: u32) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        // Unsafe block used to mimic lower-level memory editing
        unsafe {
            let ptr = self.data.as_mut_ptr().add(index);
            *ptr = new_val;
        }
        Ok(())
    }
}

// This function spawns a thread to update the data buffer at an index
// that is intentionally out of bounds.
// The vulnerability: the code incorrectly treats an error result as a
// successful update.
pub fn process_update() -> bool {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);

    // Spawn a thread where the update is attempted.
    let handle = thread::spawn(move || {
        let result = shared_clone.lock().unwrap().modify(10, 42); // index 10 is invalid for a length-10 buffer
        // Vulnerability: Inverted check of the result.
        if result.is_err() {
            // Err means update failed, but the code incorrectly treats this as success.
            true
        } else {
            false
        }
    });

    handle.join().unwrap()
}

fn main() {
    let outcome = process_update();
    if outcome {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}