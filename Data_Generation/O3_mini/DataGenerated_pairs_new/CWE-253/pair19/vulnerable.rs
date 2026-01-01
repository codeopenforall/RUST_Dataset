//////////////////////////////////////////////
// Vulnerable Code for Incorrect Return Value Check
// CWE-253: Incorrect Check of Function Return Value
//////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

// A shared manager holding a vector buffer.
struct Manager {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Manager {
    // Creates a Manager with a buffer of fixed size 10.
    fn new() -> Self {
        Manager {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    // Attempts to update an element in the buffer.
    // Vulnerability: The result of a helper function (calc_offset)
    // is interpreted incorrectly. An error return is misinterpreted
    // as a success, leading to out-of-bounds memory writes.
    fn update(&self, idx: usize, val: u32) -> Result<(), &'static str> {
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            unsafe {
                // Lock the buffer and acquire a raw pointer.
                let lock = data_clone.lock().unwrap();
                let ptr = lock.as_ptr() as *mut u32;
                // Release the lock scope.
                drop(lock);
                // Call helper that checks index bounds.
                let offset = calc_offset(idx);
                // INCORRECT CHECK:
                // If the helper returns an error (i.e. out-of-bound index),
                // the code erroneously proceeds to write to memory.
                if offset.is_err() {
                    // This leads to an unsafe out-of-bounds write.
                    *ptr.add(idx) = val;
                }
            }
        });
        handle.join().unwrap();
        Ok(())
    }

    // Returns the value stored at the given index (if available).
    fn get(&self, idx: usize) -> Option<u32> {
        let lock = self.data.lock().unwrap();
        lock.get(idx).cloned()
    }
}

// Helper function that validates the index.
// Returns Ok(index) if index is within [0, 10), else returns an error.
fn calc_offset(index: usize) -> Result<usize, &'static str> {
    if index < 10 {
        Ok(index)
    } else {
        Err("index out-of-bound")
    }
}

fn main() {
    let mgr = Manager::new();
    // Attempt to update a valid index.
    // Expected behavior: update should occur successfully.
    let _ = mgr.update(5, 100);
    // Attempt to update an invalid index.
    // Due to the incorrect return value check, this will perform an out-of-bound write.
    let _ = mgr.update(10, 200);

    // Display current state.
    if let Some(val) = mgr.get(5) {
        println!("Buffer[5] = {}", val);
    }
    if let Some(val) = mgr.get(10) {
        println!("Buffer[10] = {}", val);
    } else {
        println!("Buffer[10] is inaccessible");
    }
}