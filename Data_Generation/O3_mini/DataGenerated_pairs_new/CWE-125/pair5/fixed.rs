/*
   The corrected implementation removes the off-by-one error in the loop, ensuring that only valid elements 
   from the data array are processed. Concurrency and smart pointer usage remain unchanged.
*/
use std::sync::Arc;
use std::thread;

#[repr(C)]
struct Buffer {
    data: [i32; 4],
    // Guard field that is not meant to be processed.
    guard: i32,
}

impl Buffer {
    // The processing function now safely iterates only over valid indices.
    fn calculate(&self) -> i32 {
        let n = self.data.len();
        let mut sum = 0;
        // Unsafe block is still used here for demonstration purposes, but the loop bounds are correct.
        unsafe {
            let ptr = self.data.as_ptr();
            // Corrected loop: iterate from 0 up to but not including n.
            for i in 0..n {
                sum += *ptr.add(i);
            }
        }
        sum
    }
}

fn main() {
    // Create a Buffer with the same data and guard.
    // The expected correct sum is 10.
    let buf = Buffer { data: [1, 2, 3, 4], guard: 42 };
    // Wrap in an Arc and spawn a thread to mimic concurrent use.
    let shared = Arc::new(buf);
    let shared_clone = Arc::clone(&shared);

    let handler = thread::spawn(move || {
        shared_clone.calculate()
    });

    let res = handler.join().unwrap();
    println!("Computed sum: {}", res);
}