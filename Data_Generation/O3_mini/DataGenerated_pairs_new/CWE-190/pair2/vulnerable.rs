#![feature(core_intrinsics)]

use std::intrinsics::unchecked_mul;
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    // This function calculates the product of numbers unsafely, ignoring overflow.
    fn calculate_product(nums: &[u64]) -> Result<u64, &'static str> {
        let mut product: u64 = 1;
        // Using concurrency: divide work among threads.
        let shared = Arc::new(Mutex::new(product));
        let mut handles = vec![];

        // Each thread multiplies one number unsafely.
        for &num in nums {
            let shared_clone = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                // Lock the shared product.
                let mut val = shared_clone.lock().unwrap();
                // UNSAFE: using unchecked intrinsic multiplication can produce wraparound without error.
                unsafe {
                    *val = unchecked_mul(*val, num);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to finish.
        for handle in handles {
            handle.join().unwrap();
        }
        let final_product = *shared.lock().unwrap();
        Ok(final_product)
    }
}

fn main() {
    // Example input that causes integer overflow: 2^63 * 4 = 2^65 which overflows u64.
    let values = [9223372036854775808_u64, 4_u64];
    match Processor::calculate_product(&values) {
        Ok(result) => {
            // In release mode, unchecked multiplication will wrap around.
            println!("Computed product: {}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}