///////////////////////////////
// Vulnerable Version Example
///////////////////////////////
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
    
    // This method uses unsafe pointer arithmetic. It subtracts 1 from the requested index without
    // validating that the subtraction will remain within bounds. This can cause an underflow when
    // the input index is 0, leading to an out‐of‐bounds read.
    pub fn fetch(&self, idx: usize) -> u8 {
        unsafe {
            // Vulnerability: idx-1 may underflow and end up reading memory before the buffer.
            *self.data.get_unchecked(idx - 1)
        }
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
    
    // Spawn a thread that calls the fetch method with the given index.
    let handle = thread::spawn(move || {
        let val = shared_clone.fetch(index);
        println!("Thread Fetched value: {}", val);
    });
    
    handle.join().unwrap();
}