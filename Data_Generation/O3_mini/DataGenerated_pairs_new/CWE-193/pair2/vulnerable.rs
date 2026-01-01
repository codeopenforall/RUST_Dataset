use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        // UNSAFE: Directly mark the vector as fully initialized.
        unsafe { data.set_len(size); } // Line with potential issue: initializing memory unsafely.
        // Initialize each element; note that the unsafety combined with later off-by-one access can hide fencepost errors.
        for i in 0..size {
            data[i] = i as u8;
        }
        Self { data }
    }

    fn process(&self) -> u8 {
        // Here the code intends to fetch the last element but erroneously uses the length, which is off by one.
        unsafe {
            // Off-by-one error: attempts to read memory at index equal to the vector length.
            *self.data.get_unchecked(self.data.len())
        }
    }
}

fn main() {
    let buffer = Arc::new(Mutex::new(DataBuffer::new(10)));
    let clone = Arc::clone(&buffer);
    let handle = thread::spawn(move || {
        let guard = clone.lock().unwrap();
        guard.process()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}