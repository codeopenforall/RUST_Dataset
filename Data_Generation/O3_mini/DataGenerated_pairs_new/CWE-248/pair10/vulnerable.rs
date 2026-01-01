///////////////////////////////////////////////////////////////
// Vulnerable Code: This example mimics a scenario where an
// unsafe arithmetic operation (division by a parameter)
// is executed in a threaded context via a trait method.
// A division by zero (when input==0) will panic and is not
// caught, allowing the panic to propagate and terminate the
// process abnormally.
///////////////////////////////////////////////////////////////

use std::thread;

trait Operation {
    // Unsafe method performing an arithmetic operation.
    // No attempt is made to catch panics from invalid input.
    unsafe fn execute(&self, divisor: i32) -> i32;
}

struct Processor {
    base: i32,
}

impl Operation for Processor {
    // Vulnerable code: division by divisor without checking.
    unsafe fn execute(&self, divisor: i32) -> i32 {
        // This division will panic if divisor is zero.
        self.base / divisor
    }
}

fn perform_action(divisor: i32) {
    let proc = Processor { base: 100 };
    // Unsafe block calling the trait method.
    let result = unsafe { proc.execute(divisor) };
    println!("Computed result: {}", result);
}

fn main() {
    // Spawn a thread that triggers the dangerous operation.
    let handle = thread::spawn(|| {
        // Passing zero to cause a division-by-zero panic.
        perform_action(0);
    });
    // The join unwrap will re-propagate the panic.
    handle.join().unwrap();
}