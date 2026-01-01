/* 
   The fixed version addresses the unchecked return value by explicitly checking the result of the unsafe operation.
   If the unsafe operation fails (for input == 42), the flag is not updated, and the thread exits gracefully.
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
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            // Unsafe block calling the operation with proper result checking.
            let result = unsafe { Self::unsafe_op(input) };
            // Only update the flag if the operation succeeded.
            if result.is_ok() {
                flag.store(true, Ordering::Relaxed);
            }
        });
        handle.join().unwrap();
        self.flag.load(Ordering::Relaxed)
    }

    // The same unsafe function that returns an error for input equal to 42.
    unsafe fn unsafe_op(input: u32) -> Result<(), &'static str> {
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