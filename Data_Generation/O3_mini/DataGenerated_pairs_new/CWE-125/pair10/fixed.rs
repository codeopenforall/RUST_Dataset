////////////////////////////////////
// Corrected Version Example
////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Buffer { data }
    }
    
    // This method now validates the index before accessing the buffer.
    // By checking that idx is greater than zero and within bounds, we eliminate the
    // risk of underflow and out‐of‐bounds read.
    pub fn fetch(&self, idx: usize) -> u8 {
        if idx == 0 || idx > self.data.len() {
            panic!("Index out of bounds");
        }
        // Safe indexing is used here.
        self.data[idx - 1]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Parsing input index from command line; default is 0.
    let index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };

    let shared = Arc::new(Buffer::new(vec![10, 20, 30, 40]));
    let shared_clone = Arc::clone(&shared);
    
    // Spawn a thread that calls the fetch method with input validation.
    let handle = thread::spawn(move || {
        let val = shared_clone.fetch(index);
        println!("Thread Fetched value: {}", val);
    });
    
    handle.join().unwrap();
}