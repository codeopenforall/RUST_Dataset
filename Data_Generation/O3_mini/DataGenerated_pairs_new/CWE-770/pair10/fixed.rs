//////////////////////////////
// Corrected Version Code   //
// CWE-770 Fix: Enforce limits on resource allocation
//////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

const MAX_CAP: usize = 1000; // limit for resource allocation

trait Processor {
    fn process(&self, count: usize) -> Result<u64, String>;
}

struct Manager;

impl Manager {
    // Safe allocation with enforced limit.
    fn allocate(&self, size: usize) -> Result<Vec<u8>, String> {
        if size > MAX_CAP {
            return Err(format!(
                "Requested size {} exceeds the maximum allowed {}",
                size, MAX_CAP
            ));
        }
        // Using safe resize which checks the allocation size.
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 1); // fill with value 1
        Ok(vec)
    }
}

impl Processor for Manager {
    fn process(&self, count: usize) -> Result<u64, String> {
        // Enforce a cap on the input before proceeding
        let data = self.allocate(count)?;
        let chunk_size = if count < 4 { 1 } else { count / 4 };
        let num_threads = if count < 4 { 1 } else { 4 };
        let shared_data = Arc::new(data);
        let mut handles = Vec::new();
        let mut total: u64 = 0;

        for idx in 0..num_threads {
            let cloned_data = Arc::clone(&shared_data);
            let handle = thread::spawn(move || -> u64 {
                let start = idx * chunk_size;
                let end = if idx == num_threads - 1 {
                    cloned_data.len()
                } else {
                    start + chunk_size
                };
                let mut local_sum = 0u64;
                for i in start..end {
                    local_sum += cloned_data[i] as u64;
                }
                local_sum
            });
            handles.push(handle);
        }
        for handle in handles {
            match handle.join() {
                Ok(val) => total += val,
                Err(_) => return Err("A thread panicked".to_string()),
            }
        }
        Ok(total)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Read a user-supplied count for resource allocation.
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    let instance = Manager;
    match instance.process(count) {
        Ok(result) => println!("Final Result: {}", result),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}