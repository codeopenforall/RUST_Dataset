//////////////////////////////////////////////
// Fixed version where the off‐by‐one vulnerability is corrected.
// The unsafe fill method now writes strictly within the intended memory bounds
// by iterating from 0 to size-1. Concurrency is retained using Arc, but the
// memory write now correctly excludes the sentinel.
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u8>,
    size: usize, // logical number of elements to fill
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl Buffer {
    // Creates a new buffer with a capacity of (size + 1)
    // so that the extra slot serves as a sentinel.
    fn new(size: usize) -> Self {
        let data = vec![0u8; size + 1];
        Buffer { data, size }
    }

    // Unsafe fill writes a given value into indices 0..size.
    // Corrected loop bound avoids corrupting the sentinel.
    unsafe fn fill(&self, value: u8) {
        let ptr = self.data.as_ptr() as *mut u8;
        // Fixed: Iterates only over valid indices 0..size (excludes sentinel).
        for i in 0..self.size {
            *ptr.add(i) = value;
        }
    }

    // Provides read access for testing purposes.
    fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}

fn main() {
    let buffer = Arc::new(Buffer::new(10));
    let buffer_clone = Arc::clone(&buffer);
    // Spawn a thread to concurrently invoke the unsafe fill.
    let handle = thread::spawn(move || {
        unsafe {
            buffer_clone.fill(0xAA);
        }
    });
    unsafe {
        buffer.fill(0xAA);
    }
    handle.join().unwrap();
    println!("Finished operation");
}