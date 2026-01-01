//////////////////////// Vulnerable Code /////////////////////////////////
use std::thread;

pub trait Handler {
    fn process(&self, data: &[u64]) -> Result<u64, &'static str>;
}

pub struct ResourceManager;

impl Handler for ResourceManager {
    // This function processes the input slice by spawning a new thread for each element.
    // Each thread uses an unsafe block to double the value.
    // No input size check is enforced. An attacker providing too many items can exhaust system resources.
    fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        let mut result = 0u64;
        let mut threads = Vec::with_capacity(data.len());
        // Vulnerability: unbounded thread spawn with unsafe pointer usage.
        for &item in data {
            // Each thread processes one item concurrently.
            let handle = thread::spawn(move || {
                // Unsafe block performing pointer-based arithmetic.
                unsafe {
                    let mut val = item;
                    let ptr = &mut val as *mut u64;
                    // Double the value. This simple unsafe arithmetic mimics complex routines.
                    *ptr = *ptr * 2;
                    *ptr
                }
            });
            threads.push(handle);
        }
        // Accumulate the results.
        for th in threads {
            // If any thread panics, propagate the error.
            result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
        }
        Ok(result)
    }
}

fn main() {
    // Example input: a benign small set.
    let manager = ResourceManager;
    let data = vec![1, 2, 3, 4];
    match manager.process(&data) {
        Ok(total) => println!("Result: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}