use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Data {
    value: i32,
}

impl Data {
    // This unsafe function simulates a low-level computation.
    // It returns an error if the input is negative.
    unsafe fn compute(input: i32) -> Result<i32, &'static str> {
        let ptr = &input as *const i32;
        if *ptr < 0 {
            Err("input negative")
        } else {
            Ok(*ptr * 2)
        }
    }
}

// This publicly visible function runs the “contract” by processing a list of integers concurrently.
// It is expected to propagate errors when a negative number is provided; however, it uses unwrap_or_default()
// to silently swallow errors and return a default value (0) instead.
pub fn run_contract(inputs: &[i32]) -> Result<Vec<i32>, &'static str> {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for &input in inputs {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Vulnerability: Instead of propagating error from compute(), errors are ignored by using unwrap_or_default.
            let res = unsafe { Data::compute(input).unwrap_or_default() };
            let mut vec = shared_clone.lock().unwrap();
            vec.push(res);
        });
        handles.push(handle);
    }
    for handle in handles {
        // Join all threads; any panic here will propagate.
        handle.join().unwrap();
    }
    let vec = shared.lock().unwrap();
    Ok(vec.clone())
}

fn main() {
    let inputs = vec![-5, 10, -3, 7];
    match run_contract(&inputs) {
        Ok(results) => println!("Aggregated Results: {:?}", results),
        Err(e) => eprintln!("Error: {}", e),
    }
}