/////////////////////////////////////////////////////////////
// Complex example demonstrating an uncaught panic across threads,
// induced by an unsafe memory read in a concurrent execution.
// CWE-248: Uncaught Exception (abnormal termination) occurs when a thread panics
// and the panic is not properly caught before propagating.
/////////////////////////////////////////////////////////////
use std::thread;

trait Execute {
    fn execute(&self);
}

struct Runner;

impl Execute for Runner {
    fn execute(&self) {
        // Spawn a thread that performs an unsafe operation.
        let handle = thread::spawn(|| {
            unsafe {
                // Emulate an unsafe read from a raw pointer.
                let num_ptr = &10 as *const i32;
                let value = *num_ptr; // safe because pointer is valid, but used for simulation.
                if value == 10 {
                    // Trigger an abnormal termination by panicking.
                    panic!("Thread encountered an unrecovered panic");
                }
            }
        });
        // This unwrap will re-panic if the spawned thread panicked,
        // causing an uncaught exception propagation.
        handle.join().unwrap();
    }
}

/// Public function to execute the core logic.
/// (Used by external tests)
pub fn run_logic() {
    let runner = Runner;
    runner.execute();
}

fn main() {
    run_logic();
    println!("Execution completed (this line is never reached in failure cases)");
}