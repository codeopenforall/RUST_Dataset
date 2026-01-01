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

    // Corrected function: loop iterates strictly over valid indices.
    fn fill(&mut self) {
        let size = self.data.capacity();
        unsafe {
            // Initialize the vector with uninitialized memory.
            self.data.set_len(size);
            let ptr = self.data.as_mut_ptr();
            // Correct loop range: iterating from 0 to size (exclusive).
            for i in 0..size {
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