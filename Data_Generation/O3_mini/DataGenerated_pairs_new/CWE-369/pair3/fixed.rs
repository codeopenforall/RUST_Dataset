////////////////////////////////////////////
// Corrected Code Sample
////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Arithmetic;

impl Arithmetic {
    // Performs division using an unsafe block but now includes an explicit check to avoid division by zero.
    // When a zero divisor is detected, the function returns None.
    fn compute(numerator: i32, divisor: i32) -> Option<i32> {
        unsafe {
            let ptr: *const i32 = &divisor;
            let val = *ptr;
            if val == 0 {
                return None;
            }
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
                let res = Arithmetic::compute(num, den);
                match res {
                    Some(val) => println!("Result: {}", val),
                    None => println!("Handled division by zero error"),
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}