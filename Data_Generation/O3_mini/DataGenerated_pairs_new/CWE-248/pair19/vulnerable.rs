////////////////////////////////////////////////////////////////////////////////
// Vulnerability: This code demonstrates a scenario reminiscent of CWE-248,
// where an abnormal termination (panic) propagates due to unsafe operations
// within a concurrent thread without proper panic handling. The thread’s
// unsafe block simulates an FFI-like operation accessing a raw pointer, and
// if a dangerous condition is met, a panic occurs. The main thread then
// unconditionally unwraps the thread join result, causing the panic to propagate.
////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Processor {
    fn new() -> Self {
        // A vector with values that trigger the dangerous condition.
        Self { data: Arc::new(Mutex::new(vec![10, 20, 30])) }
    }

    fn run_internal(&self) {
        let data_ref = self.data.clone();
        let handle = thread::spawn(move || {
            // Unsafe block simulating an FFI call or an operation on raw pointers.
            unsafe {
                // Lock the shared data and create a raw pointer to its buffer.
                let guard = data_ref.lock().unwrap();
                let ptr = guard.as_ptr();
                // If the first element is less than 100, trigger panic.
                // This simulates a logic bug in unsafe code leading to abnormal termination.
                if *ptr < 100 {
                    panic!("Abnormal termination: unsafe access condition met");
                }
            }
            // Additional business logic (omitted).
            let _ = 42; // placeholder
        });
        // Vulnerability: Unwrapping the thread join result causes the panic
        // from the spawned thread to propagate and crash the program.
        handle.join().unwrap();
    }
}

// Public entry point used by the test oracle.
pub fn app() {
    let proc = Processor::new();
    proc.run_internal();
}

fn main() {
    app();
}