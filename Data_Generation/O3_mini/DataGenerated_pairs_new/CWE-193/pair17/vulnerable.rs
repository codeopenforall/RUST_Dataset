use std::sync::{Arc, Mutex};
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Buffer {
        Buffer {
            data: Vec::with_capacity(size),
        }
    }

    // Core function that fills the buffer.
    // Vulnerability: Off-by-one error in loop iteration causes writing one element past the allocated memory.
    fn fill(&mut self) {
        let size = self.data.capacity();
        unsafe {
            // Pretend to initialize the vector with uninitialized memory.
            self.data.set_len(size);
            let ptr = self.data.as_mut_ptr();
            // Off-by-one error: iterating from 0 to size inclusive writes one element too many.
            for i in 0..=size { // Vulnerable loop: should be 0..size
                *ptr.add(i) = i as u32;
            }
        }
    }

    fn sum(&self) -> u32 {
        self.data.iter().sum()
    }
}

// Public API for processing that can be used by external tests.
pub fn process() -> u32 {
    let mut buffer = Buffer::new(10);
    buffer.fill();
    buffer.sum()
}

fn main() {
    // Runs the public process API and prints out the result.
    let result = process();
    println!("Result: {}", result);

    // Simulate concurrent accesses.
    let shared = Arc::new(Mutex::new(Buffer {
        data: Vec::with_capacity(10),
    }));
    {
        // Fill the buffer concurrently (for illustration: not strictly needed by test oracle).
        let mut temp = shared.lock().unwrap();
        temp.fill();
    }
    let mut handles = Vec::new();
    for _ in 0..5 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let lock = shared_clone.lock().unwrap();
            lock.sum()
        }));
    }
    for handle in handles {
        let res = handle.join().unwrap();
        println!("Thread result: {}", res);
    }
}