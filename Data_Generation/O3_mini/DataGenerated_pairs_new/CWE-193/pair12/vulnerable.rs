/////////////////////////////////////////////
// Vulnerable Code Example - Off-by-One Issue //
/////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    // Holds a vector of unsigned 32-bit integers.
    // Memory is allocated with capacity only.
    data: Vec<u32>,
}

impl DataBuffer {
    // Creates a new DataBuffer with the given number of elements.
    // Note: Using with_capacity means the vector has uninitialized memory.
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        // Unsafe: inform the vector it is fully "initialized" even though it is not.
        unsafe {
            data.set_len(size);
        }
        Self { data }
    }

    // Fills the buffer with the index value.
    // Vulnerability: off-by-one error in loop boundary.
    unsafe fn populate(&mut self) {
        // The intention was to iterate exactly over all valid indices.
        // Mistakenly, the code uses 0..=len creating a fencepost error.
        for i in 0..=self.data.len() {
            // This uses unchecked access which will write one element past the allocated range.
            *self.data.get_unchecked_mut(i) = i as u32;
        }
    }

    // Computes the sum of all elements.
    fn total(&self) -> u32 {
        self.data.iter().sum()
    }
}

fn compute() -> u32 {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);
    
    let handler = thread::spawn(move || {
        let mut guard = shared_clone.lock().expect("failed to lock");
        // Unsafe population of the vector. Off-by-one error occurs here.
        unsafe {
            guard.populate();
        }
    });
    
    handler.join().expect("Thread panicked");
    let guard = shared.lock().expect("failed to lock after join");
    guard.total()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}