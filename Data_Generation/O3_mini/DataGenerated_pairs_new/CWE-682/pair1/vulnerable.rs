/*
This Rust program demonstrates a calculation function that computes a derived value 
by using an unsafe block and concurrent access via a Mutex-protected shared resource. 
However, due to an operator precedence mistake, the arithmetic is computed incorrectly.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Calculation {
    value: i64,
}

impl Calculation {
    // Computes a derived result using an unsafe block.
    // Intended to compute (value + factor) / 2.
    // Vulnerability: The division is applied only to 'factor' because of incorrect operator precedence.
    fn compute(&self, factor: i64) -> i64 {
        unsafe {
            let ptr: *const i64 = &self.value;
            // Incorrect due to operator precedence: equivalent to *ptr + (factor / 2)
            *ptr + factor / 2
        }
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Calculation { value: 10 }));
    let operation_factor = 5; // Expected: (10+5)/2 = 7, but vulnerability computes 10 + (5/2) = 12.
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result;
        {
            let calc = shared_clone.lock().unwrap();
            result = calc.compute(operation_factor);
        }
        result
    });
    let final_result = handle.join().unwrap();
    println!("Computed result: {}", final_result);
}