use std::sync::{Arc, Mutex};
use std::thread;

pub struct MemoryHandler {
    data: Vec<u8>,
}

impl MemoryHandler {
    pub fn new() -> Self {
        // Allocate a fixed-size buffer of 100 bytes.
        MemoryHandler { data: vec![0u8; 100] }
    }

    // This method updates the buffer at the given index with the provided value.
    // It lacks proper bounds checking: if idx is greater than or equal to the length,
    // the unsafe pointer arithmetic writes outside the allocated memory.
    pub fn update(&mut self, idx: usize, value: u8) -> bool {
        unsafe {
            // Obtain a raw mutable pointer to the data.
            let ptr = self.data.as_mut_ptr();
            // Vulnerability: No check is performed to ensure 'idx' is within bounds.
            // This may write beyond the allocated 100-byte buffer.
            *ptr.add(idx) = value;
        }
        true
    }

    // Fetch the value at the given index (no check for brevity).
    pub fn fetch(&self, idx: usize) -> u8 {
        unsafe { *self.data.as_ptr().add(idx) }
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let mut threads = vec![];

    // Spawn 10 threads that update positions in the buffer.
    // When idx becomes >= 100, writes will be out-of-bounds.
    for i in 0..10 {
        let h = Arc::clone(&handler);
        threads.push(thread::spawn(move || {
            let mut mgr = h.lock().unwrap();
            // Create an index that intentionally goes out-of-bound when i >= 5.
            let idx = 95 + i;
            // The update method does not restrict operations within bounds.
            let _ = mgr.update(idx, 42);
        }));
    }

    for th in threads {
        th.join().unwrap();
    }

    let mgr = handler.lock().unwrap();
    // This prints the value at index 95 (which may be incorrect due to corruption).
    println!("Buffer value at index 95: {}", mgr.fetch(95));
}