//////////////////////
// Vulnerable Code
//////////////////////
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn run_threads(count: usize) -> Result<usize, &'static str> {
    // Allocate unbounded threads without any throttling.
    // This loop can spawn an extreme number of threads when count is not capped.
    let shared = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(count);

    for i in 0..count {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Unsafe block used to simulate low-level pointer manipulation in real-world FFI or concurrent code.
            unsafe {
                // Cast to mutable pointer and update the atomic.
                let ptr = &*(&*s as *const AtomicUsize as *mut AtomicUsize);
                // Adding the loop index as a faux workload.
                ptr.fetch_add(i, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        // If a thread panics, bubble up the error.
        handle.join().map_err(|_| "Thread join failed")?;
    }
    Ok(shared.load(Ordering::Relaxed))
}

fn main() {
    // Read the count from command-line arguments. Default is 10.
    let args: Vec<String> = env::args().collect();
    // For example, passing a huge number (e.g., 2000 or more) triggers resource exhaustion.
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(10);
    match run_threads(count) {
        Ok(total) => println!("Computation result: {}", total),
        Err(e) => eprintln!("Error: {}", e),
    }
}