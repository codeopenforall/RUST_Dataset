/////////////////// BEGIN FIXED CODE ///////////////////
use std::env;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_TASKS: usize = 100; // Cap on the allowed number of tasks.

/// A helper structure that “allocates” resources by launching threads
/// and performing unsafe memory initialization. In this fixed version,
/// an explicit cap prevents resource exhaustion.
struct ResourceHandler;

impl ResourceHandler {
    // This method safely spawns tasks only if within allowed threshold.
    fn run_tasks(&self, num: usize) {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::with_capacity(num);

        for i in 0..num {
            let count_ref = Arc::clone(&counter);
            // Spawning threads within a controlled limit.
            let handle = thread::spawn(move || {
                unsafe {
                    let mut buf: Vec<u8> = Vec::with_capacity(1024);
                    let ptr = buf.as_mut_ptr();
                    std::ptr::write_bytes(ptr, 0, 1024);
                }
                count_ref.fetch_add(1, Ordering::Relaxed);
                println!("Task {} completed", i);
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        println!("Total tasks completed: {}", counter.load(Ordering::Relaxed));
    }
}

/// Public API executing the tasks only if the requested number does not exceed the limit.
pub fn execute(num: usize) -> Result<(), &'static str> {
    // Validate the user-supplied task count against the maximum allowed.
    if num > MAX_TASKS {
        return Err("Error: Too many tasks requested");
    }
    let handler = ResourceHandler;
    handler.run_tasks(num);
    Ok(())
}

fn main() {
    // Read command line argument and interpret as number of tasks.
    let args: Vec<String> = env::args().collect();
    let tasks: usize = if args.len() > 1 {
        // In case of a parsing failure, use a safe default.
        args[1].parse().unwrap_or(MAX_TASKS)
    } else {
        MAX_TASKS
    };
    // Execute task creation; if the request exceeds the limit, report the error.
    if let Err(e) = execute(tasks) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
/////////////////// END FIXED CODE ///////////////////