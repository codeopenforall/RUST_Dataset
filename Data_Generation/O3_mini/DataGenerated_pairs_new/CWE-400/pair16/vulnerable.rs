/* 
This program implements a resource handler that spawns a new thread for each task,
using an unsafe block to manipulate a raw pointer. The design flaw is that it does not impose
any bound on the number of threads it spawns. Under a very high iteration count,
this lack of backpressure can lead to uncontrolled resource consumption (CWE-400).
*/
use std::thread;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::env;

fn process(limit: u32) -> Result<u64, &'static str> {
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for i in 0..limit {
        let total = total.clone();
        // The following unsafe block manipulates a raw pointer.
        // Moreover, each loop iteration spawns a new thread without checking if 'limit' is too high.
        unsafe {
            let handle = thread::spawn(move || {
                let mut local = i as u64;
                // Unsafe raw pointer conversion and dereference.
                let ptr = &mut local as *mut u64;
                *ptr = *ptr + 1;
                total.fetch_add(*ptr, Ordering::Relaxed);
            });
            handles.push(handle);
        }
    }
    for h in handles {
        h.join().map_err(|_| "Thread join error")?;
    }
    Ok(total.load(Ordering::Relaxed))
}

fn main() {
    // Accept an optional limit parameter. Without proper checks, an attacker could supply a huge number
    // and force the program to spawn an excessive number of threads.
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