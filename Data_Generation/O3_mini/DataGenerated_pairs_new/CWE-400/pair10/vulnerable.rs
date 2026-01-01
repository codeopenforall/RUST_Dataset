//////////////////////////////////////////////////////////
// Realistic vulnerable Rust code sample
// Issue: Uncontrolled Resource Consumption (CWE-400)
// This code spawns one thread per request based solely on
// a user-provided argument with no limit imposed. In a
// production scenario, a malicious actor can supply a very
// high number to exhaust system resources.
//////////////////////////////////////////////////////////

use std::env;
use std::thread;

static mut GLOBAL: u64 = 0;

pub fn compute(n: u64) -> u64 {
    // Reset the global counter.
    unsafe {
        GLOBAL = 0;
    }
    let mut handles = vec![];
    // Each iteration spawns a new thread without backpressure.
    // This loop is vulnerable because it does not place any upper bound
    // on the number of threads created.
    for _ in 0..n {
        handles.push(thread::spawn(|| {
            // Unsafe block with unsynchronized shared state modification.
            unsafe {
                let val = GLOBAL;
                // Simulate some computation.
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