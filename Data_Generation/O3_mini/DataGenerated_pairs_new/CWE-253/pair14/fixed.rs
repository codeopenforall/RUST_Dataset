////////////////////////////////////////////////////////////////
// Corrected Code Sample for CWE-253
////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

// Unsafe function simulating an external API call.
// Returns 0 on success and -1 on error.
unsafe fn external_call(input: i32) -> i32 {
    if input > 0 {
        0
    } else {
        -1
    }
}

// A structure representing an operation.
struct Operator {
    value: i32,
}

impl Operator {
    fn new(val: i32) -> Self {
        Operator { value: val }
    }

    // Correct check: now interprets 0 as success.
    fn execute(&self) -> bool {
        let ret = unsafe { external_call(self.value) };
        // Fixed: properly treating 0 as success.
        if ret == 0 {
            true
        } else {
            false
        }
    }
}

fn main() {
    // Wrap Operator in Arc and Mutex to simulate a realistic concurrent environment.
    let op = Arc::new(Mutex::new(Operator::new(5)));
    let mut threads = vec![];

    for _ in 0..5 {
        let op_clone = Arc::clone(&op);
        let handle = thread::spawn(move || {
            let instance = op_clone.lock().unwrap();
            if instance.execute() {
                println!("Operation succeeded");
            } else {
                println!("Operation failed");
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}