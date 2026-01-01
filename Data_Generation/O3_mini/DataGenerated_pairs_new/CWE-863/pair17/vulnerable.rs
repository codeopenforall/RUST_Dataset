/////////////////////////////////////////////////////////////
// Vulnerable Code: Resource exhaustion due to unbounded thread creation.
// Compile and run with: cargo run --release <n>
// Using unsafe and concurrency in a realistic implementation.
/////////////////////////////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1)
        .expect("Missing input parameter")
        .parse()
        .expect("Invalid number");
    match execute(n) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn execute(n: usize) -> Result<u64, &'static str> {
    // Create an atomic accumulator.
    let total = Arc::new(AtomicU64::new(0));
    // Allocate a handle container with user-supplied capacity.
    let mut handles = Vec::with_capacity(n);
    // UNSAFE block used to mimic low-level operations,
    // and no limits are enforced on thread creation.
    unsafe {
        for i in 0..n {
            let total_clone = Arc::clone(&total);
            let h = thread::spawn(move || {
                // Unsafe pointer-like behavior: concurrent update without throttling.
                total_clone.fetch_add(i as u64, Ordering::SeqCst);
            });
            handles.push(h);
        }
    }
    // Wait for each thread to complete.
    for h in handles {
        h.join().map_err(|_| "Thread panicked")?;
    }
    Ok(total.load(Ordering::SeqCst))
}