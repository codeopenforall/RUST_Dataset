////////////////////////////////////////////////////////////////////////////////
// Vulnerability: Off‐by‐one Heap Overwrite via Unsafe Pointer Arithmetic in a Concurrent Context
// This code uses unsafe blocks and multithreading to write to a vector. An off‐by‐one error
// in the iteration range (using “0..len+1” instead of “0..len”) writes one element past the end
// of the vector’s allocated region, leading to undefined behavior that may be exploited in a real
// world scenario.
////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Accumulator {
    // Creates a vector with the specified capacity but unsafely sets its length.
    // This “initialization” does not write any valid data.
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size); } // Unsafe: memory uninitialized.
        Self { data: Arc::new(Mutex::new(vec)) }
    }

    // Concurrently writes to each element of the vector.
    // OFF-BY-ONE ERROR: The loop iterates from 0 to len (inclusive) instead of exclusive,
    // causing an out-of-bound write in one of the worker threads.
    fn process(&self) {
        let len = self.data.lock().unwrap().len();
        let mut handles = Vec::new();
        for i in 0..len+1 {
            let arc_clone = Arc::clone(&self.data);
            handles.push(thread::spawn(move || {
                let mut vec = arc_clone.lock().unwrap();
                // Unsafe write: using pointer arithmetic which does not check bounds.
                unsafe {
                    let ptr = vec.as_mut_ptr();
                    *ptr.add(i) = i as i32; // Vulnerable: when i == len, this writes past the allocated region.
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Computes the sum of the vector elements.
    fn total(&self) -> i32 {
        let vec = self.data.lock().unwrap();
        vec.iter().sum()
    }
}

fn main() {
    let aggregator = Accumulator::new(10);
    aggregator.process();
    println!("Sum: {}", aggregator.total());
}