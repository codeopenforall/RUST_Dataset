///////////////////////////////////////////////////////////////
// Vulnerable version: Unsafe buffer writes allow an out‐of‐bounds write (CWE-787)
// This code uses unsafe pointer arithmetic and a misuse of Vec::set_len to
// simulate a real-world vulnerability.
///////////////////////////////////////////////////////////////
use std::sync::{Mutex, Arc};
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Creates a new Buffer with uninitialized elements.
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        // Unsafely assume that the vector’s capacity becomes its length.
        // (This is safe for some types but here it starts uninitialized.)
        unsafe {
            data.set_len(size);
        }
        Buffer { data }
    }
    
    // Unsafe write without bounds checking.
    // Writes to the provided index regardless of whether it is in bounds.
    unsafe fn store(&mut self, index: usize, value: u32) {
        // Direct out-of-bounds pointer write using unchecked indexing.
        *self.data.get_unchecked_mut(index) = value;
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
    
    // This function attempts to store a value at an index equal to the length,
    // triggering an out-of-bound write.
    fn run(&mut self) {
        unsafe {
            // Vulnerability: writing at index equal to the current length.
            // This is an out-of-bound write since valid indices are 0..length-1.
            self.buffer.store(self.buffer.data.len(), 100);
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