//////////////////////////////
// Fixed Version -- Race Condition CWE-362
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

fn run_app() -> i32 {
    // Use Arc and Mutex to ensure proper synchronization.
    let shared = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // Lock is acquired per increment to avoid race condition.
                let mut counter = shared_clone.lock().unwrap();
                *counter += 1;
            }
        });
        handles.push(handle);
    }
    // Join all threads.
    for h in handles {
        h.join().unwrap();
    }
    // Return the final value
    let counter = shared.lock().unwrap();
    *counter
}

fn main() {
    let result = run_app();
    println!("Final counter value: {}", result);
}