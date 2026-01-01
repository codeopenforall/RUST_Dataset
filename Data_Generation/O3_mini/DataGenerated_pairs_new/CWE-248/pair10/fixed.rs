///////////////////////////////////////////////////////////////
// Fixed Code: This version addresses the vulnerability by
// wrapping the unsafe arithmetic call in a std::panic::catch_unwind,
// thereby preventing an uncaught panic from propagating.
// The error is handled gracefully and logged.
///////////////////////////////////////////////////////////////

use std::thread;
use std::panic;

trait Operation {
    // Unsafe method performing an arithmetic operation.
    unsafe fn execute(&self, divisor: i32) -> i32;
}

struct Processor {
    base: i32,
}

impl Operation for Processor {
    // Safe in the sense that the caller will catch any panic.
    unsafe fn execute(&self, divisor: i32) -> i32 {
        // This may panic if divisor is zero.
        self.base / divisor
    }
}

fn perform_action(divisor: i32) -> Result<i32, &'static str> {
    let proc = Processor { base: 100 };
    // Catch potential panics caused by unsafe operation.
    let result = panic::catch_unwind(|| {
        unsafe { proc.execute(divisor) }
    });
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Operation failed: division by zero or invalid input"),
    }
}

fn main() {
    // Spawn a thread to safely handle the operation.
    let handle = thread::spawn(|| {
        match perform_action(0) {
            Ok(result) => println!("Computed result: {}", result),
            Err(e) => println!("Error encountered: {}", e),
        }
    });
    // Join the thread; since panics are caught, join returns normally.
    handle.join().expect("Thread panicked unexpectedly");
}