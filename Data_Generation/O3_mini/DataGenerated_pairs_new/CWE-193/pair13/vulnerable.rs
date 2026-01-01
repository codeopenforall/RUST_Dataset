//////////////////////////////////////////
// Vulnerable Code: Off-by-One Indexing Bug
//////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Create a buffer with capacity n.
    fn new(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        unsafe {
            // UNSAFE: Mark the vector as having n elements even though they are uninitialized.
            data.set_len(n);
        }
        Buffer { data }
    }

    // Method to fill the buffer with values 1,2,...,n.
    fn populate(&mut self) {
        let n = self.data.len();
        // Off-by-one error: using 0..=n iterates one extra time.
        for i in 0..=n {
            unsafe {
                // UNSAFE: Calculating pointer offset without proper bounds check.
                let ptr = self.data.as_mut_ptr().add(i);
                // Writing value (i+1) causing an out-of-bounds write when i == n.
                *ptr = (i + 1) as u32;
            }
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