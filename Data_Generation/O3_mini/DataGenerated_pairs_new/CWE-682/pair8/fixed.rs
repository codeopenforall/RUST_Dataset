use std::thread;
use std::sync::Arc;

pub mod calc {
    use super::*;

    pub struct Processor;

    impl Processor {
        pub fn new() -> Self {
            Processor
        }

        // Safely computes the error percentage with the correct arithmetic,
        // while still using unsafe pointer arithmetic and concurrency to emulate real-world complexity.
        pub fn execute(&self, data: &[u64]) -> u64 {
            if data.len() < 2 {
                return 0;
            }
            let ptr = data.as_ptr();
            let total: u64;
            let measured: u64;
            unsafe {
                total = *ptr;         // retrieve total value
                measured = *ptr.add(1); // retrieve measured value
            }

            let total_arc = Arc::new(total);
            let measured_arc = Arc::new(measured);

            let tot_clone = Arc::clone(&total_arc);
            let meas_clone = Arc::clone(&measured_arc);

            let handle = thread::spawn(move || {
                // Correct calculation: first subtract then multiply and divide.
                // Correctly computes: ((total - measured) * 100) / total
                (*tot_clone - *meas_clone) * 100 / *tot_clone
            });

            handle.join().unwrap()
        }
    }
}

fn main() {
    let processor = calc::Processor::new();
    // Example input: total = 100, measured = 80.
    let input_data = vec![100u64, 80u64];
    let outcome = processor.execute(&input_data);
    println!("Outcome: {}", outcome);
}