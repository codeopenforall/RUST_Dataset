//////////////////////////////
// Vulnerable Code Sample
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct BufferManager {
    buffer: Vec<u8>,
}

impl BufferManager {
    fn new() -> Self {
        // Create a vector with 10 elements.
        let mut data = Vec::with_capacity(10);
        data.resize(10, 0); // Length becomes 10.
        BufferManager { buffer: data }
    }

    // Incorrect update: off-by-one error causing an out-of-bounds write.
    fn apply(&mut self, idx: usize, value: u8) -> Result<(), String> {
        unsafe {
            // Vulnerability: instead of writing to buffer[idx], writing to buffer[idx + 1]
            // For example, when idx == 9 (last valid index), this writes to index 10.
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx + 1) = value;
        }
        Ok(())
    }

    fn sum(&self) -> u32 {
        self.buffer.iter().map(|&b| b as u32).sum()
    }
}

fn main() {
    // Set up shared state for concurrent usage.
    let manager = BufferManager::new();
    let shared = Arc::new(Mutex::new(manager));

    let handles: Vec<_> = (0..2)
        .map(|i| {
            let shared_ref = Arc::clone(&shared);
            // Each thread attempts to update what is thought to be a valid index.
            thread::spawn(move || {
                let mut mgr = shared_ref.lock().unwrap();
                // Intentionally updating index 9 (last valid index),
                // but the off-by-one bug writes to index 10 (out-of-bound).
                mgr.apply(9, 42 + i as u8).unwrap();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let mgr = shared.lock().unwrap();
    println!("Sum: {}", mgr.sum());
}