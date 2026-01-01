//////////////////////////////////////////////////////////////
// Fixed Code: Correct handling of the update result value
// This version uses the same structure and concurrency pattern but
// correctly treats an error result as a failure. When the update function
// returns an error due to an out‐of‐bound index, the code now properly
// reports the failure.
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

// This function spawns a thread to update the shared data buffer at an
// intentionally invalid index, but now the return value is checked properly.
pub fn process_update() -> bool {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        let result = shared_clone.lock().unwrap().modify(10, 42);
        // Correct check: Only consider the update successful if no error occurred.
        if result.is_ok() {
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