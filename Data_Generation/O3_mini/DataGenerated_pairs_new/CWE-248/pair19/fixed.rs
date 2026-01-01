////////////////////////////////////////////////////////////////////////////////
// Correction: This version fixes the abnormal termination issue by properly
// catching panics within the spawned thread. It wraps the unsafe operation within
// std::panic::catch_unwind, then gracefully propagates any errors. The main thread
// inspects the result of the thread execution and handles errors without panicking.
////////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::panic;

struct Runner {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Runner {
    fn new() -> Self {
        // Same shared vector as before.
        Self { data: Arc::new(Mutex::new(vec![10, 20, 30])) }
    }

    fn run_internal(&self) -> Result<(), &'static str> {
        let data_ref = self.data.clone();
        // Spawn a thread and use catch_unwind to recover from a panic.
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                unsafe {
                    let guard = data_ref.lock().unwrap();
                    let ptr = guard.as_ptr();
                    if *ptr < 100 {
                        panic!("Abnormal termination: unsafe access condition met");
                    }
                }
                // Additional business logic (omitted).
                let _ = 42; // placeholder
            });
            result.map_err(|_| ())
        });
        // Instead of unwrapping blindly, check the thread's result.
        match handle.join() {
            Ok(inner_result) => inner_result.map_err(|_| "Error: Panic caught in thread"),
            Err(_) => Err("Error: Thread panicked unexpectedly"),
        }
    }
}

// Public entry point used by the test oracle.
pub fn app() {
    let runner = Runner::new();
    // We deliberately ignore the result here after handling it gracefully.
    let _ = runner.run_internal();
}

fn main() {
    app();
}