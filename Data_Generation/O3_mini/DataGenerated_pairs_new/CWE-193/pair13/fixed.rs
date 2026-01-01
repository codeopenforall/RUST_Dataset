//////////////////////////////////////////
// Corrected Code: Off-by-One Issue Fixed
//////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Create a buffer with n initialized elements.
    fn new(n: usize) -> Self {
        // SAFETY: Using vec! macro to ensure the vector is properly initialized.
        let data = vec![0; n];
        Buffer { data }
    }

    // Fill the buffer with values 1,2,...,n.
    fn populate(&mut self) {
        let n = self.data.len();
        // Correct iteration: 0..n to fill exactly n elements.
        for i in 0..n {
            // Safe indexing since vector is fully initialized.
            self.data[i] = (i + 1) as u32;
        }
    }

    // Compute the sum concurrently.
    fn concurrent_sum(&self) -> u32 {
        let shared = Arc::new(self.data.clone());
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut total = 0;
            for &val in shared_clone.iter() {
                total += val;
            }
            total
        });
        handle.join().unwrap()
    }
}

fn main() {
    let mut buf = Buffer::new(10);
    buf.populate();
    let total = buf.concurrent_sum();
    println!("Total: {}", total);
}