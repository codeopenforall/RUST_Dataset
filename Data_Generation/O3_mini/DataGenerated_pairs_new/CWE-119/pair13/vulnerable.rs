//////////////////////////////////////////////
// Vulnerable Implementation (CWE-119)
// This code incorrectly checks memory bounds in a buffer update,
// allowing an index equal to the vector length. This unsafe pointer
// arithmetic may write past the allocated memory when multiple threads
// concurrently update the buffer.
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
        // Vulnerable: Incorrect boundary check allows 'index' equal to length.
        if index <= self.data.len() {           // (Line 15)
            unsafe {
                // Unsafe pointer arithmetic without proper bounds check.
                let ptr = self.data.as_mut_ptr().add(index);  // (Line 17)
                *ptr = val;                                   // (Line 18)
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

    // Spawn 10 threads. Some iterations (when i==5,6,..) will trigger an update
    // with an invalid index (equal to or greater than the buffer length).
    for i in 0..10 {
        let buf_clone = Arc::clone(&buf);
        let handle = thread::spawn(move || {
            let mut locked = buf_clone.lock().unwrap();
            // For i==5, the condition (index <= len) passes, resulting in an out-of-bound write.
            let _ = locked.update(i, i as u8);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Execution completed in vulnerable build.");
}