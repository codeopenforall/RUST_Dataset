////////////////////////////////////////////
// Vulnerable Code Sample
////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Arithmetic;

impl Arithmetic {
    // Performs division using an unsafe block without verifying that the divisor is non-zero.
    // The function returns an Option, but if the divisor is zero the operation will panic.
    fn compute(numerator: i32, divisor: i32) -> Option<i32> {
        unsafe {
            let ptr: *const i32 = &divisor;
            let val = *ptr;
            // Vulnerability: division by zero is not checked.
            Some(numerator / val)
        }
    }
}

fn main() {
    // A collection of (numerator, divisor) tuples; one tuple has a zero divisor.
    let data = Arc::new(Mutex::new(vec![(100, 5), (50, 0), (30, 3)]));
    let mut handles = vec![];

    // Spawn several threads to simulate concurrent arithmetic operations.
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let tasks = data_clone.lock().unwrap();
            for &(num, den) in tasks.iter() {
                // In case den == 0, this call will panic due to unchecked division.
                let res = Arithmetic::compute(num, den);
                match res {
                    Some(val) => println!("Result: {}", val),
                    None => println!("Unexpected None returned"),
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}