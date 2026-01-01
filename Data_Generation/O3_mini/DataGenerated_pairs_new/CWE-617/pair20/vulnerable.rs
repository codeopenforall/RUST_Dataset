//////////////////////////////////////////////////////////////
// CWE-617 Example: Reachable Assertion in Concurrent Unsafe Code
// This code compiles with unsafe Rust constructs. It uses an unsafe
// update routine that manipulates the internal state of a shared
// structure concurrently. The state “threshold” is updated from an
// attacker-controlled input without proper validation, and if the
// value exceeds the expected safe maximum (10) a reachable assertion
// is triggered causing a panic (and potential denial-of-service).
//////////////////////////////////////////////////////////////

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Data {
    threshold: u32,
}

impl Data {
    fn new(val: u32) -> Self {
        Data { threshold: val }
    }

    // Unsafe update uses raw pointer manipulation allowing an attacker
    // to set the threshold to an arbitrary value.
    unsafe fn modify(&mut self, val: u32) {
        let ptr = self as *mut Data;
        (*ptr).threshold = val;
    }
}

fn gather_input(input: u32) -> Arc<Mutex<Data>> {
    let shared = Arc::new(Mutex::new(Data::new(5)));
    {
        let shared_clone = Arc::clone(&shared);
        // Spawn a thread to simulate asynchronous update
        thread::spawn(move || {
            let mut data_guard = shared_clone.lock().unwrap();
            unsafe {
                data_guard.modify(input);
            }
        })
        .join()
        .unwrap();
    }
    shared
}

fn main() {
    // Read an attacker-influenced input from command-line arguments.
    let args: Vec<String> = env::args().collect();
    let input: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(5)
    } else {
        5
    };
    let shared = gather_input(input);
    let data = shared.lock().unwrap();
    // Reachable assertion that will panic when data.threshold > 10.
    // This is the vulnerability that triggers DoS if an attacker supplies
    // a large input.
    assert!(data.threshold <= 10, "Threshold exceeded safe limit");
    println!("Threshold is safe: {}", data.threshold);
}