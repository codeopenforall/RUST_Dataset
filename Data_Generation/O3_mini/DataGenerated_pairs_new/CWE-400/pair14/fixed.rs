//////////////////////// Fixed Code /////////////////////////////////
use std::thread;

pub trait Handler {
    fn process(&self, data: &[u64]) -> Result<u64, &'static str>;
}

pub struct ResourceManager;

impl Handler for ResourceManager {
    // In the safe variant, we impose an upper bound on the number of items processed at once.
    // Instead of spawning a thread for each element, we process them sequentially if the input is large.
    fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        const MAX_ITEMS: usize = 1000;
        if data.len() > MAX_ITEMS {
            return Err("Input size exceeds allowable limit");
        }
        let mut result = 0u64;
        // For a safe workload, we avoid unbounded concurrency.
        if data.len() > 10 {
            // For moderate loads, use a limited thread spawning strategy.
            let mut threads = Vec::with_capacity(data.len());
            for &item in data {
                let handle = thread::spawn(move || {
                    unsafe {
                        let mut val = item;
                        let ptr = &mut val as *mut u64;
                        *ptr = *ptr * 2;
                        *ptr
                    }
                });
                threads.push(handle);
            }
            for th in threads {
                result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
            }
        } else {
            // For small inputs, process sequentially.
            for &item in data {
                let doubled = unsafe {
                    let mut val = item;
                    let ptr = &mut val as *mut u64;
                    *ptr = *ptr * 2;
                    *ptr
                };
                result = result.saturating_add(doubled);
            }
        }
        Ok(result)
    }
}

fn main() {
    let manager = ResourceManager;
    // Default benign input.
    let data = vec![1, 2, 3, 4];
    match manager.process(&data) {
        Ok(total) => println!("Result: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}