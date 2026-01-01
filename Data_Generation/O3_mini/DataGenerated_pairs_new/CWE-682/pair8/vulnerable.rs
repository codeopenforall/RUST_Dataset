use std::thread;
use std::sync::Arc;

pub mod calc {
    use super::*;

    pub struct Processor;

    impl Processor {
        pub fn new() -> Self {
            Processor
        }

        // This function expects at least two elements in the slice: the total and the measurement.
        // It uses unsafe raw pointer arithmetic and concurrency.
        pub fn execute(&self, data: &[u64]) -> u64 {
            if data.len() < 2 {
                return 0;
            }
            let ptr = data.as_ptr();
            let total: u64;
            let measured: u64;
            unsafe {
                total = *ptr;       // retrieve total value (first element)
                measured = *ptr.add(1); // retrieve measured value (second element)
            }

            // Emulate concurrent processing using smart pointers.
            let total_arc = Arc::new(total);
            let measured_arc = Arc::new(measured);

            let tot_clone = Arc::clone(&total_arc);
            let meas_clone = Arc::clone(&measured_arc);

            let handle = thread::spawn(move || {
                // Vulnerability: Incorrect arithmetic due to operator precedence.
                // Intended calculation: ((total - measured) * 100) / total.
                // Erroneous calculation: total - (measured / total * 100)
                *tot_clone - (*meas_clone / *tot_clone * 100)
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