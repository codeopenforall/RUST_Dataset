//////////////////////////////
// Corrected Code Sample
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct BufferManager {
    buffer: Vec<u8>,
}

impl BufferManager {
    fn new() -> Self {
        // Create a vector with exactly 10 elements.
        let mut data = Vec::with_capacity(10);
        data.resize(10, 0); // Properly initialize vector with length 10.
        BufferManager { buffer: data }
    }

    // Correct update: uses proper index to access the buffer and checks bounds.
    fn apply(&mut self, idx: usize, value: u8) -> Result<(), String> {
        if idx < self.buffer.len() {
            self.buffer[idx] = value;
            Ok(())
        } else {
            Err("Index out of bounds".into())
        }
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
            // Each thread updates a valid index using the corrected update.
            thread::spawn(move || {
                let mut mgr = shared_ref.lock().unwrap();
                // Update the intended element (index 9).
                mgr.apply(9, 42 + i as u8).expect("Update failed");
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let mgr = shared.lock().unwrap();
    println!("Sum: {}", mgr.sum());
}