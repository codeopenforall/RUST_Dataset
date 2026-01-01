/* 
This program corrects the uncontrolled resource consumption by enforcing a strict upper bound (MAX_LIMIT)
on the number of iterations (and threads) that can be spawned. Additionally, it avoids unnecessary unsafe
raw pointer manipulations by using safe arithmetic methods.
*/
use std::thread;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::env;

const MAX_LIMIT: u32 = 1000; // Safe threshold for task count.

fn process(limit: u32) -> Result<u64, &'static str> {
    // Enforce safe upper bound to prevent resource exhaustion.
    if limit > MAX_LIMIT {
        return Err("Limit exceeds safe threshold");
    }
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for i in 0..limit {
        let total = total.clone();
        let handle = thread::spawn(move || {
            let local = i as u64;
            // Use safe arithmetic without unsafe pointer manipulation.
            let local = local.wrapping_add(1);
            total.fetch_add(local, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().map_err(|_| "Thread join error")?;
    }
    Ok(total.load(Ordering::Relaxed))
}

fn main() {
    // Accept an optional limit parameter. The program now validates the input against MAX_LIMIT.
    let args: Vec<String> = env::args().collect();
    let limit: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        10  // Default iteration count.
    };
    match process(limit) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}