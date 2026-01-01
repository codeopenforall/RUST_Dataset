////////////////////////////////////////////////////////////////
// Fixed implementation: Proper Bounds Checking Preventing OOB Writes
////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    // Creates a new DataBuffer, initializing the vector with zeros.
    fn new(size: usize) -> Self {
        DataBuffer { data: vec![0; size] }
    }

    // This safe version verifies that the index is within bounds before updating.
    // Out-of-bound updates are ignored (or could also return an error).
    fn process(&mut self, index: usize, value: u8) -> u32 {
        if index < self.data.len() {
            self.data[index] = value;
        }
        self.data.iter().map(|&v| v as u32).sum()
    }
}

fn main() {
    // Arc and Mutex are used as in the vulnerable version, ensuring thread-safe sharing.
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        let mut buf = shared_clone.lock().unwrap();
        // The update with an out-of-bound index is safely ignored.
        let res = buf.process(10, 42);
        println!("Thread result: {}", res);
    });

    {
        let mut buf = shared.lock().unwrap();
        // An in-bound update which correctly updates the buffer.
        let res = buf.process(5, 13);
        println!("Main thread result: {}", res);
    }
    handle.join().unwrap();
}