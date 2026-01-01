////////////////////////////////////////////////////////////
// Vulnerable Code: Division without checking for zero.
//
// Uses unsafe arithmetic and concurrency primitives,
// returning a Result type. However, when the denominator
// is zero, the unsafe division (100 / denominator)
// will panic instead of returning an error.
// This mimics a real-world flaw where unchecked division
// operations lead to runtime failures.
////////////////////////////////////////////////////////////

use std::env;
use std::process;
use std::thread;
use std::sync::Arc;

struct Calculator;

impl Calculator {
    // Performs division in an unsafe block.
    // Signature returns a Result for API consistency, but
    // does not perform any zero-check.
    fn execute(&self, denominator: i32) -> Result<i32, &'static str> {
        // Unsafe division prone to panic if denominator is zero.
        unsafe {
            Ok(100 / denominator)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };

    let calc = Calculator;
    let data = Arc::new(input);
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        // Simulate concurrent retrieval of the parameter.
        *data_clone
    });
    let thread_val = handle.join().unwrap();

    // This call will panic if thread_val is zero.
    let result = calc.execute(thread_val).unwrap();
    println!("Output: {}", result);
}