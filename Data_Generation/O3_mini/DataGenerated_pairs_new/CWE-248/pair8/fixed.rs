//////////////////////////////
// Corrected Code Sample    //
// CWE-248: Handled Panic     //
//////////////////////////////
use std::thread;
use std::panic::{catch_unwind, AssertUnwindSafe};

struct Manager;

impl Manager {
    fn new() -> Self {
        Self {}
    }

    // Modified method to catch panics within the spawned thread.
    fn process(&self) -> Result<(), String> {
        let th = thread::spawn(|| {
            unsafe {
                let mut value: i32 = 0;
                let raw_ptr: *mut i32 = &mut value;
                // Wrap the unsafe operation in a catch_unwind to prevent propagation.
                let result = catch_unwind(AssertUnwindSafe(|| {
                    if *raw_ptr == 0 {
                        panic!("abnormal termination: unexpected condition encountered");
                    }
                }));
                // Translate the result into a meaningful error message if a panic occurred.
                result.map_err(|_| "panic caught inside thread".to_string())
            }
        });
        // Join the thread and handle any potential join error.
        match th.join() {
            Ok(inner_result) => inner_result,
            Err(_) => Err("thread panicked and join error occurred".to_string()),
        }
    }
}

fn run_system() {
    let mgr = Manager::new();
    // Instead of letting panic propagate, process() returns an error that is handled gracefully.
    if let Err(e) = mgr.process() {
        // Proper error reporting instead of letting the process crash.
        println!("Error handled: {}", e);
    }
}

fn main() {
    // run_system() now handles panics gracefully using catch_unwind.
    run_system();
}