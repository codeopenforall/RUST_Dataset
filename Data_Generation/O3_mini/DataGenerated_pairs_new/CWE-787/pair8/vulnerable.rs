////////////////////////////////////////////////////////////////////////////////
// This Rust program demonstrates an unsafe implementation that can write
// out-of-bound by means of unchecked pointer arithmetic and misuse of set_len.
// It uses a simple BufferManager struct that holds a vector of u32 values.
// The update method deliberately avoids bounds checking by performing an unsafe
// write to the memory location pointed to by the vector's internal pointer and
// then erroneously adjusts the vector's length. This code also spawns a thread
// to simulate a concurrent context.
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

    // Updates an element at index `idx` by writing the value `val`.
    // This unsafely writes past the end of the allocated buffer when idx >= len.
    // It then calls set_len with idx+1, which can cause the vector metadata
    // to not match the actual memory allocated.
    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx) = val;          // Vulnerable: no bound check on pointer arithmetic.
            self.buffer.set_len(idx + 1); // Vulnerable: incorrectly sets length beyond capacity.
        }
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
        // Trigger an out-of-bound update (index 5, while initial len() is 5).
        let _ = manager.update(5, 42);
        println!("Sum: {}", manager.sum());
    });
    handle.join().unwrap();
}