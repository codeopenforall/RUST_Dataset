//////////////////////////////////////////
// Vulnerable Module
//////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Calculator;

impl Calculator {
    // This unsafe function uses raw pointers and performs division
    // without checking for a zero divisor.
    pub unsafe fn compute(&self, a: i32, b: i32) -> i32 {
        // Mimic unsafe numeric operations using raw pointers.
        let a_ptr: *const i32 = &a;
        let b_ptr: *const i32 = &b;
        let numerator = *a_ptr;
        let divisor = *b_ptr; // No check: if divisor is zero, this will panic at runtime.
        numerator / divisor  // Vulnerability: Division by zero may occur here.
    }
}

// A public function used by external callers (e.g. tests). This
// simply delegates to Calculator::compute. In our vulnerable code,
// a divisor of 0 will cause a runtime panic.
pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    calc.compute(a, b)
}

fn main() {
    // Initialize shared Calculator and test data.
    let calc = Arc::new(Calculator);
    // Two pairs: first one is valid; second one has zero divisor causing panic.
    let data = Arc::new(Mutex::new(vec![(10, 2), (10, 0)])); 
    
    let mut handles = Vec::new();
    // Spawn a thread per computation.
    for _ in 0..data.lock().unwrap().len() {
        let calc_clone = calc.clone();
        let data_clone = data.clone();
        let handle = thread::spawn(move || {
            let pair = data_clone.lock().unwrap().pop().unwrap();
            unsafe {
                // This call may panic if the divisor is zero.
                let res = calc_clone.compute(pair.0, pair.1);
                println!("Computed result: {}", res);
                res
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        // Join threads (one of these may panic due to division by zero).
        let _ = handle.join();
    }
}