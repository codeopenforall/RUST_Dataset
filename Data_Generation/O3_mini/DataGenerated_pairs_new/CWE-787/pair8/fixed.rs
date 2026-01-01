////////////////////////////////////////////////////////////////////////////////
// This Rust program shows the corrected version that prevents out-of-bounds writes.
// The BufferManager struct now verifies that the provided index is within bounds before
// performing the update. The safe indexing provided by Rust is used and no unsafe block is needed.
// If an out-of-range index is supplied, an appropriate error is returned.
////////////////////////////////////////////////////////////////////////////////

use std::vec::Vec;
use std::thread;

struct BufferManager {
    buffer: Vec<u32>,
}

impl BufferManager {
    // Initializes the buffer with five elements.
    fn new() -> Self {
        BufferManager {
            buffer: vec![1, 1, 1, 1, 1],
        }
    }

    // Safely updates an element at index `idx` by writing the value `val`.
    // If the index is out-of-range, it returns an error and leaves the buffer unchanged.
    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.buffer.len() {
            return Err("Index out of bounds");
        }
        self.buffer[idx] = val;
        Ok(())
    }

    // Computes the sum of all elements in the buffer.
    fn sum(&self) -> u32 {
        self.buffer.iter().sum()
    }
}

fn main() {
    let mut manager = BufferManager::new();
    // Spawn a thread to simulate concurrent execution.
    let handle = thread::spawn(move || {
        // Attempt to update an out-of-bound index. This will now be safely rejected.
        match manager.update(5, 42) {
            Ok(_) => println!("Unexpected update. Buffer state may be corrupted."),
            Err(e) => println!("Error: {}", e),
        }
        println!("Sum: {}", manager.sum());
    });
    handle.join().unwrap();
}