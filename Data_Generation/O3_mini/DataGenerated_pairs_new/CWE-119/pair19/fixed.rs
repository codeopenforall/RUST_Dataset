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
    // It now properly checks that the index is within valid bounds. If idx is out-of-range,
    // the operation is safely rejected.
    pub fn update(&mut self, idx: usize, value: u8) -> bool {
        if idx < self.data.len() {
            unsafe {
                // Although still using unsafe pointer arithmetic,
                // the bounds-check ensures the operation is safe.
                *self.data.get_unchecked_mut(idx) = value;
            }
            true
        } else {
            // Index is out-of-bound, so we reject the update.
            false
        }
    }

    // Fetch the value at the given index if within bounds, or return 0.
    pub fn fetch(&self, idx: usize) -> u8 {
        if idx < self.data.len() {
            unsafe { *self.data.get_unchecked(idx) }
        } else {
            0
        }
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let mut threads = vec![];

    // Spawn 10 threads that update the buffer safely.
    for i in 0..10 {
        let h = Arc::clone(&handler);
        threads.push(thread::spawn(move || {
            let mut mgr = h.lock().unwrap();
            let idx = 95 + i;
            // If idx is out-of-bound, the update returns false and the operation is not performed.
            let result = mgr.update(idx, 42);
            if !result {
                eprintln!("Attempted to update invalid index: {}", idx);
            }
        }));
    }

    for th in threads {
        th.join().unwrap();
    }

    let mgr = handler.lock().unwrap();
    // This prints the value at index 95, which remains valid.
    println!("Buffer value at index 95: {}", mgr.fetch(95));
}