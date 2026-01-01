/*
This version corrects the arithmetic calculation error.
By introducing explicit parentheses, we ensure the intended computation (value + factor) / 2 is carried out.
The structure, concurrency, and unsafe block usage remain to mimic real-world patterns while fixing the vulnerability.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Calculation {
    value: i64,
}

impl Calculation {
    // Corrected compute function that properly calculates (value + factor) / 2.
    fn compute(&self, factor: i64) -> i64 {
        unsafe {
            let ptr: *const i64 = &self.value;
            // Correct calculation with parentheses: first sum then divide.
            (*ptr + factor) / 2
        }
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Calculation { value: 10 }));
    let operation_factor = 5; // Expected: (10+5)/2 = 7
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