///////////////////////////////////////////////////////////////
// Fixed version: Use safe buffer initialization and strict bounds checks.
// This code corrects the vulnerability by initializing the vector properly
// and using bounds-checked access before writing.
///////////////////////////////////////////////////////////////
use std::sync::{Mutex, Arc};
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Creates a new Buffer with pre-initialized zero values.
    fn new(size: usize) -> Self {
        // Using vec! macro to initialize all elements to 0.
        let data = vec![0; size];
        Buffer { data }
    }
    
    // Safe write with explicit bounds checking.
    fn store_safe(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
}

trait Summable {
    fn total(&self) -> u32;
}

struct Worker {
    buffer: Buffer,
}

impl Summable for Worker {
    fn total(&self) -> u32 {
        self.buffer.data.iter().sum()
    }
}

impl Worker {
    fn new(size: usize) -> Self {
        Worker { buffer: Buffer::new(size) }
    }
    
    // The corrected run method only writes within valid bounds.
    fn run(&mut self) {
        // Fix: write to index 0 which is within bounds.
        if let Err(e) = self.buffer.store_safe(0, 100) {
            eprintln!("Error storing value: {}", e);
        }
    }
}

fn main() {
    let worker = Arc::new(Mutex::new(Worker::new(5)));
    let worker_clone = Arc::clone(&worker);
    
    let handle = thread::spawn(move || {
        let mut w = worker_clone.lock().unwrap();
        w.run();
    });
    
    handle.join().unwrap();
    
    let total = worker.lock().unwrap().total();
    println!("Computed total: {}", total);
}