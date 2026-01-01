/////////////////// BEGIN VULNERABLE CODE ///////////////////
use std::env;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A helper structure that “allocates” resources by launching threads
/// and performing unsafe memory initialization. There is no cap on the
/// number of tasks created.
struct ResourceHandler;

impl ResourceHandler {
    // This method spawns tasks (threads) without any limit verification.
    fn run_tasks(&self, num: usize) {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::with_capacity(num);

        for i in 0..num {
            let count_ref = Arc::clone(&counter);
            // Spawning threads without throttling or resource limits.
            let handle = thread::spawn(move || {
                // Unsafe block simulating low-level resource initialization.
                unsafe {
                    // Allocate a Vec with a dummy capacity (simulate fixed-size resource usage)
                    let mut buf: Vec<u8> = Vec::with_capacity(1024);
                    // Use raw pointer to write into the buffer without initializing it.
                    let ptr = buf.as_mut_ptr();
                    // Write 1024 zero bytes (simulate a sensitive operation)
                    std::ptr::write_bytes(ptr, 0, 1024);
                }
                // Increase a counter (simulated work)
                count_ref.fetch_add(1, Ordering::Relaxed);
                println!("Task {} completed", i);
            });
            handles.push(handle);
        }
        // Wait for all threads to complete.
        for handle in handles {
            let _ = handle.join();
        }
        println!("Total tasks completed: {}", counter.load(Ordering::Relaxed));
    }
}

/// Public API executing the tasks based on the provided count.
/// NOTE: No authorization or input limit check is performed here.
pub fn execute(num: usize) -> Result<(), &'static str> {
    let handler = ResourceHandler;
    handler.run_tasks(num);
    Ok(())
}

fn main() {
    // Read the first command line argument as the number of tasks.
    let args: Vec<String> = env::args().collect();
    let tasks: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(100)
    } else {
        100
    };
    // Execute task creation regardless of input magnitude.
    let _ = execute(tasks);
}
/////////////////// END VULNERABLE CODE ///////////////////