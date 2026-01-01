//////////////////////////////////////////////
// Fixed Code for Incorrect Return Value Check
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
    // Fix: The helper function's return value is properly checked.
    // Only if the index is valid (helper returns Ok), the unsafe update is performed.
    // Otherwise, the function returns an error.
    fn update(&self, idx: usize, val: u32) -> Result<(), &'static str> {
        // Validate the index first.
        let offset = calc_offset(idx)?;
        let data_clone = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            unsafe {
                let lock = data_clone.lock().unwrap();
                let ptr = lock.as_ptr() as *mut u32;
                drop(lock);
                // Since offset is valid, perform the update.
                *ptr.add(offset) = val;
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
// Returns Ok(index) if index is within bounds, else returns an error.
fn calc_offset(index: usize) -> Result<usize, &'static str> {
    if index < 10 {
        Ok(index)
    } else {
        Err("index out-of-bound")
    }
}

fn main() {
    let mgr = Manager::new();
    // Update a valid index.
    if let Err(e) = mgr.update(5, 100) {
        println!("Error updating index 5: {}", e);
    }
    // Attempt to update an invalid index.
    if let Err(e) = mgr.update(10, 200) {
        println!("Properly rejected update at index 10: {}", e);
    }
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