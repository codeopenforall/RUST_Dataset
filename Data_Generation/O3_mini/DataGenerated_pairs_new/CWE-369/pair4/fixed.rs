//////////////////////////////////////////
// Corrected Module
//////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Calculator;

impl Calculator {
    // This unsafe function performs division but includes a check to avoid
    // dividing by zero. If b is zero, it returns an error.
    pub unsafe fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        // Use raw pointers to mimic an unsafe numeric operation.
        let a_ptr: *const i32 = &a;
        let b_ptr: *const i32 = &b;
        let numerator = *a_ptr;
        let divisor = *b_ptr;
        Ok(numerator / divisor)
    }
}

// A public function for external callers (e.g. tests). It calls the Calculator
// and, in case of a zero divisor, returns a safe fallback value (0).
pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    match calc.compute(a, b) {
        Ok(val) => val,
        Err(err) => {
            println!("Error encountered: {}", err);
            0  // Safe fallback value when division is not permitted.
        }
    }
}

fn main() {
    // Initialize shared Calculator and test data.
    let calc = Arc::new(Calculator);
    // Two data pairs: one valid and one with a zero divisor.
    let data = Arc::new(Mutex::new(vec![(10, 2), (10, 0)])); 

    let mut handles = Vec::new();
    // Spawn threads to concurrently perform division.
    for _ in 0..data.lock().unwrap().len() {
        let calc_clone = calc.clone();
        let data_clone = data.clone();
        let handle = thread::spawn(move || {
            let pair = data_clone.lock().unwrap().pop().unwrap();
            unsafe {
                // Use the safe compute: if a zero divisor is encountered,
                // an error is handled and a fallback value is returned.
                let res = match calc_clone.compute(pair.0, pair.1) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Error encountered: {}", err);
                        0
                    }
                };
                println!("Computed result: {}", res);
                res
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.join();
    }
}