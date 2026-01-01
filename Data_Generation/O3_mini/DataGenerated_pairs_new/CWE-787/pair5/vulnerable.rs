////////////////////////////////////////////////////////////////
// Vulnerable implementation: Out-of-Bounds Write via Unsafe Code
////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    // Creates a new buffer with the given size, initializing all elements to 0.
    fn new(size: usize) -> Self {
        DataBuffer { data: vec![0; size] }
    }

    // This function attempts to update the buffer at a given index.
    // It unsafely writes to the buffer without checking bounds and
    // then misuses set_len to artificially increase the length of the vector.
    fn process(&mut self, index: usize, value: u8) -> u32 {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            // Vulnerability: writing outside the allocated bounds if index == data.len()
            *ptr.add(index) = value;
            // Vulnerability: improperly increases the length without actual allocation.
            self.data.set_len(self.data.len() + 1);
        }
        self.data.iter().map(|&v| v as u32).sum()
    }
}

fn main() {
    // Use Arc and Mutex to allow safe sharing among threads.
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);

    // Spawn a thread that performs an out-of-bound update.
    let handle = thread::spawn(move || {
        let mut buf = shared_clone.lock().unwrap();
        // Intentional out-of-bound write: index equals the original length (10).
        let res = buf.process(10, 42);
        println!("Thread result: {}", res);
    });

    // In the main thread, perform an in-bound update.
    {
        let mut buf = shared.lock().unwrap();
        let res = buf.process(5, 13);
        println!("Main thread result: {}", res);
    }
    handle.join().unwrap();
}