use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // This function computes the sum by iterating over the requested range using an unsafe unchecked access.
    // If the caller provides a range that exceeds the length of the data,
    // the use of get_unchecked leads to an out-of-bounds read (CWE-125).
    pub fn compute_sum(&self, start: usize, count: usize) -> Result<u32, &'static str> {
        let mut sum = 0;
        unsafe {
            // The following loop does not ensure that start + i is within bounds.
            for i in 0..count {
                // POTENTIAL FLAW: Unchecked read without verifying that start+i is valid.
                sum += *self.data.get_unchecked(start + i);
            }
        }
        Ok(sum)
    }
}

fn main() {
    let buffer = Arc::new(Buffer { data: vec![1, 2, 3, 4] });
    let buf_clone = Arc::clone(&buffer);

    // Spawn a thread that attempts to compute a sum using an index range that goes out-of-bound.
    let handle = thread::spawn(move || {
        // Requesting 5 elements from a 4-element vector: undefined behavior may occur.
        let result = buf_clone.compute_sum(0, 5);
        match result {
            Ok(sum) => sum,
            Err(e) => {
                eprintln!("Error: {}", e);
                0
            }
        }
    });

    // In the main thread, compute a safe sum with a proper iteration.
    let safe_sum: u32 = buffer.data.iter().sum();
    let unsafe_sum = handle.join().unwrap();
    println!("Safe sum: {}, Computed sum: {}", safe_sum, unsafe_sum);
}