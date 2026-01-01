/////////////////////////////////////////////////////////////////
// Vulnerability demonstration: CWE-770 – Unbounded Resource Allocation
// This version accepts an input count and spawns that many threads without
// imposing any limit. It also uses an unsafe block for pointer manipulation
// to mimic real-world unsafe usage patterns.
/////////////////////////////////////////////////////////////////

use std::env;
use std::thread;
use std::ptr;
use std::sync::{Arc, Mutex};

struct ResourceManager;

impl ResourceManager {
    // run_tasks spawns 'count' threads to perform a trivial increment.
    // There is no cap on the count, so a very high count can exhaust system resources.
    fn run_tasks(&self, count: usize) -> Result<usize, &'static str> {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for i in 0..count {
            let counter_clone = Arc::clone(&counter);
            // Unsafe block to simulate pointer arithmetic usage.
            unsafe {
                // Convert loop index to a raw pointer and read from it.
                let dummy = i as *const usize;
                let _ = ptr::read(dummy);
            }
            let handle = thread::spawn(move || {
                // Simulate a small work unit.
                {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }
        // Join all threads.
        for handle in handles {
            handle.join().map_err(|_| "Thread panicked")?;
        }
        let final_count = *counter.lock().unwrap();
        Ok(final_count)
    }
}

fn main() {
    // Read the desired number of threads from the command-line.
    // Defaults to 100 if no argument is provided.
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(100)
    } else {
        100
    };
    let manager = ResourceManager;
    match manager.run_tasks(count) {
        Ok(n) => println!("Completed {} tasks.", n),
        Err(e) => println!("Error: {}", e),
    }
}