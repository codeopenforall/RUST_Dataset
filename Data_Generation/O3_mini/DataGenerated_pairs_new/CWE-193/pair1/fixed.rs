////////////////////////////////////////////////////////////////////////////////
// Correction: Eliminating the Off‐by‐One Error and Using Safe Initialization
// In this version, the vector is safely resized, and the iteration uses a correct range.
// The off‐by‐one vulnerability is removed by iterating over 0..len (exclusive).
////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Accumulator {
    // Safely initializes the vector with valid data (zeros).
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0); // Safe initialization: vector now contains zeros.
        Self { data: Arc::new(Mutex::new(vec)) }
    }

    // Concurrently writes to each element of the vector using a correct iteration range.
    fn process(&self) {
        let len = self.data.lock().unwrap().len();
        let mut handles = Vec::new();
        // Corrected range: iterates over valid indices 0..len
        for i in 0..len {
            let arc_clone = Arc::clone(&self.data);
            handles.push(thread::spawn(move || {
                let mut vec = arc_clone.lock().unwrap();
                // Safe assignment using indexing, which performs bounds checking.
                vec[i] = i as i32;
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