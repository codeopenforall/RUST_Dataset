/* 
   This code simulates a vulnerability where a returned error from an unsafe operation is ignored.
   The unsafe function “unsafe_op” returns an error when the input equals 42 (a boundary condition).
   In the thread spawned in run(), the return value is not checked and the shared state is updated unconditionally,
   even if the unsafe operation failed.
*/
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

struct Executor {
    flag: Arc<AtomicBool>,
}

impl Executor {
    fn new() -> Self {
        Executor { flag: Arc::new(AtomicBool::new(false)) }
    }

    fn run(&self, input: u32) -> bool {
        // Spawn a new thread to simulate concurrency
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            // Unsafe block calling a function that may return an error.
            // The return value is deliberately discarded.
            unsafe {
                let _ = Self::unsafe_op(input);
            }
            // The flag is updated irrespective of the outcome.
            flag.store(true, Ordering::Relaxed);
        });
        handle.join().unwrap();
        self.flag.load(Ordering::Relaxed)
    }

    // Unsafe function simulating an operation which can fail for a specific input.
    unsafe fn unsafe_op(input: u32) -> Result<(), &'static str> {
        // Simulate a failure when input equals the critical value.
        if input == 42 {
            Err("Operation encountered an error")
        } else {
            Ok(())
        }
    }
}

fn process_input(input: u32) -> bool {
    let exec = Executor::new();
    exec.run(input)
}

fn main() {
    let input = 42;
    let outcome = process_input(input);
    println!("Outcome: {}", outcome);
}