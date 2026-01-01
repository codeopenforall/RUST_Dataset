//////////////////////////////
// Vulnerability: Division by Zero in Unsafe Code
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Calculator;

impl Calculator {
    // This method performs division using an unsafe block.
    // It does not check for a zero divisor, resulting in a potential division-by-zero panic.
    fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        unsafe {
            // Vulnerable use of raw pointer dereferencing to read the divisor.
            // If b is zero, a division-by-zero occurs causing a panic.
            let ptr: *const i32 = &b;
            let divisor = *ptr;
            Ok(a / divisor)
        }
    }
}

fn main() {
    let calc = Arc::new(Calculator);
    let calc_clone = Arc::clone(&calc);
    let join_handle = thread::spawn(move || {
        // Here we intentionally use zero as the divisor to illustrate the vulnerability.
        calc_clone.compute(100, 0)
    });
    // The join will either return Ok(Result) or panic if the thread panicked.
    let res = join_handle.join().expect("Thread panicked unexpectedly.");
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}