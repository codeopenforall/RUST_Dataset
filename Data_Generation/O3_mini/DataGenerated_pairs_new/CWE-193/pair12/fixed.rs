/////////////////////////////////////////////
// Fixed Code Example - Off-by-One Corrected //
/////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    // Holds a vector of unsigned 32-bit integers.
    // Memory is allocated and initialized safely.
    data: Vec<u32>,
}

impl DataBuffer {
    // Creates a new DataBuffer with the given number of elements;
    // the vector is resized to be fully initialized.
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        // Safely initialize all elements to 0.
        data.resize(size, 0);
        Self { data }
    }

    // Fills the buffer with the index value.
    // The loop correctly iterates within the allocated bounds.
    fn populate(&mut self) {
        // Correctly iterate over valid indices.
        for i in 0..self.data.len() {
            self.data[i] = i as u32;
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
        // Populate the vector safely.
        guard.populate();
    });
    
    handler.join().expect("Thread panicked");
    let guard = shared.lock().expect("failed to lock after join");
    guard.total()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}