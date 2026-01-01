////////////////////////////////////////////////////////////////
// Vulnerable Code Sample for CWE-253
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

    // The vulnerability: Incorrectly checks the external function’s return value.
    // It treats a return value of -1 (actually an error) as success,
    // and any other value (i.e. the valid 0 on success) as an error.
    fn execute(&self) -> bool {
        let ret = unsafe { external_call(self.value) };
        // Vulnerable check: misinterpreting -1 as a successful outcome.
        if ret == -1 {
            true  // Wrong: returning success when external_call fails.
        } else {
            false // Wrong: treating the successful return (0) as failure.
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