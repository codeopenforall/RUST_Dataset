use std::sync::Arc;
use std::thread;

struct Locker {
    data: Arc<Vec<u64>>,
}

impl Locker {
    fn new() -> Self {
        // A fixed-size vector holding ten elements.
        Locker { data: Arc::new(vec![0; 10]) }
    }

    // Now returns a Result to handle out-of-bound indices gracefully.
    fn update(&self, index: usize, value: u64) -> Result<(), &'static str> {
        // Validate the index before proceeding.
        if index >= self.data.len() {
            return Err("Index out of range");
        }
        unsafe {
            // Obtain a mutable pointer after ensuring the index is valid.
            let ptr = self.data.as_ptr() as *mut u64;
            *ptr.add(index) = value;
        }
        Ok(())
    }
}

fn main() {
    let locker = Locker::new();
    // Use a safe index to update; in production the caller should handle errors properly.
    if let Err(err) = locker.update(5, 42) {
        eprintln!("Failed to update: {}", err);
    } else {
        println!("Execution completed.");
    }
}