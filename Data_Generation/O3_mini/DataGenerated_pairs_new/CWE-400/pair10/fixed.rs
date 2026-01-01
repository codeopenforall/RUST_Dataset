//////////////////////////////////////////////////////////
// Realistic fixed Rust code sample
// Fix: Enforce an upper bound on the number of threads created
// to prevent uncontrolled resource consumption (CWE-400).
//////////////////////////////////////////////////////////

use std::env;
use std::thread;

static mut GLOBAL: u64 = 0;
const MAX_THREADS: u64 = 1000; // Define an upper limit for spawned threads

pub fn compute(n: u64) -> u64 {
    // Reset the global counter.
    unsafe {
        GLOBAL = 0;
    }
    // Enforce a cap on the allowed thread count.
    let count = if n > MAX_THREADS { MAX_THREADS } else { n };
    let mut handles = vec![];
    // Only spawn up to 'count' threads.
    for _ in 0..count {
        handles.push(thread::spawn(|| {
            // Still using an unsafe block (for simulation purposes),
            // but the overall resource consumption is now bounded.
            unsafe {
                let val = GLOBAL;
                let _dummy = (0..1000).fold(0, |acc, x| acc + x);
                GLOBAL = val + 1;
            }
        }));
    }
    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe { GLOBAL }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Parse the first argument as the number of threads to spawn.
    let n = args.get(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let result = compute(n);
    println!("Result: {}", result);
}