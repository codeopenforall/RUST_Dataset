use std::sync::{Arc, Mutex};
use std::thread;
struct Processor;
impl Processor {
    fn calculate_product(nums: &[u64]) -> Result<u64, &'static str> {
        let mut product: u64 = 1;
        let mut partials = vec![];
        let mut handles = vec![];
        for &num in nums {
            let handle = thread::spawn(move || -> u64 { num });
            handles.push(handle);
        }
        for handle in handles {
            partials.push(handle.join().unwrap());
        }
        for num in partials {
            product = product.checked_mul(num).ok_or("overflow detected")?;
        }
        Ok(product)
    }
}
fn main() {
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
