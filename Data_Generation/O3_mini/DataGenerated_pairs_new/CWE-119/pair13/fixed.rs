//////////////////////////////////////////////
// Corrected Implementation (CWE-119 Fix)
// This code fixes the incorrect memory bounds check by ensuring that the index
// is strictly less than the vector length before performing an unsafe pointer update.
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }

    pub fn update(&mut self, index: usize, val: u8) -> Result<(), &'static str> {
        // Fixed: Correct boundary check ensures the index is strictly within bounds.
        if index < self.data.len() {           // (Line 15)
            unsafe {
                let ptr = self.data.as_mut_ptr().add(index);
                *ptr = val;
            }
            Ok(())
        } else {
            Err("index out-of-bound")
        }
    }

    pub fn read(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_ptr().add(index);
                Ok(*ptr)
            }
        } else {
            Err("index out-of-bound")
        }
    }
}

fn main() {
    let buf = Arc::new(Mutex::new(Buffer::new(5)));
    let mut handles = vec![];

    // Spawn 10 threads that update the buffer concurrently.
    // Only valid indices (0 to 4) are accepted; invalid ones result in an error.
    for i in 0..10 {
        let buf_clone = Arc::clone(&buf);
        let handle = thread::spawn(move || {
            let mut locked = buf_clone.lock().unwrap();
            // For invalid indices, update returns an error instead of writing out-of-bound.
            let _ = locked.update(i, i as u8);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Execution completed in fixed build.");
}