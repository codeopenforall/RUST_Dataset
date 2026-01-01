////////////////////////////////////////////////////////////
// This corrected version addresses the vulnerability by
// properly initializing the vector and adding bounds checking
// in the trait implementation.
////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataPool {
    buffer: Vec<u8>,
}

trait Modifier {
    // Modifies the buffer at the given index by writing the provided value.
    // Returns Ok(()) if the modification is successful, or an error if the index is invalid.
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str>;
}

impl Modifier for DataPool {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        // Check if the index is within the valid bounds.
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out-of-bounds")
        }
    }
}

impl DataPool {
    fn new(size: usize) -> Self {
        // Safely initialize the buffer with zeros.
        let buffer = vec![0u8; size];
        DataPool { buffer }
    }
}

fn main() {
    // Create a shared DataPool object with a buffer of size 10.
    let pool = Arc::new(Mutex::new(DataPool::new(10)));

    // Spawn a thread to perform an in-bound modification.
    let pool_clone = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        let mut dp = pool_clone.lock().unwrap();
        dp.modify(5, 42).expect("In-bound modification must succeed");
    });
    handle.join().unwrap();

    let mut dp = pool.lock().unwrap();
    // Attempt an out-of-bound modification.
    // Instead of corrupting memory, this call returns an error.
    match dp.modify(11, 99) {
        Ok(_) => println!("Unexpectedly modified out-of-bound index"),
        Err(e) => println!("Failed modification as expected: {}", e),
    }
    println!("Final buffer: {:?}", dp.buffer);
}