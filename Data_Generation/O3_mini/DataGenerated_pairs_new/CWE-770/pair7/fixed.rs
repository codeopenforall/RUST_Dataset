/////////////////////////////////////////////////////////////////
// Secure version: Implements proper resource limits to prevent 
// unbounded thread creation. Here, a maximum threshold is enforced.
// The unsafe block remains as a simulation of legacy unsafe usage.
/////////////////////////////////////////////////////////////////

use std::env;
use std::thread;
use std::ptr;
use std::sync::{Arc, Mutex};

const MAX_THREADS: usize = 200; // Upper limit for thread creation.

struct ResourceManager;

impl ResourceManager {
    // run_tasks now checks that the requested count does not exceed MAX_THREADS.
    fn run_tasks(&self, count: usize) -> Result<usize, &'static str> {
        if count > MAX_THREADS {
            return Err("Excessive resource request");
        }
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for i in 0..count {
            let counter_clone = Arc::clone(&counter);
            // Unsafe block retained for compatibility, though its use remains minimal.
            unsafe {
                let dummy = i as *const usize;
                let _ = ptr::read(dummy);
            }
            let handle = thread::spawn(move || {
                {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "Thread panicked")?;
        }
        let final_count = *counter.lock().unwrap();
        Ok(final_count)
    }
}

fn main() {
    // Read the desired thread count from command-line arguments.
    // Default is 100.
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