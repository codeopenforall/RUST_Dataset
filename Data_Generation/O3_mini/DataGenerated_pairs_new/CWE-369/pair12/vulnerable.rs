use std::sync::Arc;
use std::thread;
use std::panic;

struct Calculator;

impl Calculator {
    // Performs division without checking for a zero divisor.
    fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe {
            numerator / denominator  // Vulnerability: unchecked division may panic if denominator is zero.
        }
    }
    
    // Public wrapper that catches panics and converts them into a Result.
    pub fn safe_compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        let res = panic::catch_unwind(|| self.compute(numerator, denominator));
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(String::from("Panic occurred")),
        }
    }
}

fn main() {
    let calc = Arc::new(Calculator);
    let calc_ref = Arc::clone(&calc);

    let handle = thread::spawn(move || {
        // The input 0 as divisor will trigger a divide-by-zero in the unsafe block.
        calc_ref.compute(100, 0)
    });

    // This will panic at runtime due to division by zero.
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
}