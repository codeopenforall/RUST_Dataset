use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    // This function calculates the product of numbers safely by checking for overflow.
    fn calculate_product(nums: &[u64]) -> Result<u64, &'static str> {
        let mut product: u64 = 1;
        // Using concurrency: divide work among threads.
        // Instead of performing multiplication concurrently on shared mutable state unsafely,
        // we collect intermediate results and combine them sequentially.
        let mut partials = vec![];

        // Spawn threads to return individual numbers (simulate work distribution).
        let mut handles = vec![];
        for &num in nums {
            let handle = thread::spawn(move || -> u64 { num });
            handles.push(handle);
        }
        for handle in handles {
            partials.push(handle.join().unwrap());
        }

        // Combine the results sequentially with overflow checks.
        for num in partials {
            product = product.checked_mul(num).ok_or("overflow detected")?;
        }
        Ok(product)
    }
}

fn main() {
    // Example input that causes integer overflow: 2^63 * 4 should trigger an overflow.
    let values = [9223372036854775808_u64, 4_u64];
    match Processor::calculate_product(&values) {
        Ok(result) => {
            println!("Computed product: {}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}