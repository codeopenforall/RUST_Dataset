//////////////////////
// Fixed Code
//////////////////////
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

const MAX_ALLOWED: usize = 1000;

fn run_threads(count: usize) -> Result<usize, &'static str> {
    // Cap the count to prevent resource exhaustion.
    if count > MAX_ALLOWED {
        return Err("Provided count exceeds the maximum allowed threshold");
    }
    
    let shared = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(count);

    for i in 0..count {
        let s = Arc::clone(&shared);
        // Use safe concurrency constructs without unsafe pointer manipulation.
        let handle = thread::spawn(move || {
            // Safe atomic operation without unsafe code.
            s.fetch_add(i, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread join failed")?;
    }
    Ok(shared.load(Ordering::Relaxed))
}

fn main() {
    // Read the count from command-line arguments. Default is 10.
    let args: Vec<String> = env::args().collect();
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(10);
    match run_threads(count) {
        Ok(total) => println!("Computation result: {}", total),
        Err(e) => eprintln!("Error: {}", e),
    }
}