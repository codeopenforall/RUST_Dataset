use std::sync::Arc;
use std::thread;

struct Calculator;

impl Calculator {
    // Performs division with an explicit check to avoid dividing by zero.
    fn compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        if denominator == 0 {
            return Err(String::from("Division by zero error"));
        }
        // Even though the division is inside an unsafe block, the pre-check guarantees safety.
        let result = unsafe { numerator / denominator };
        Ok(result)
    }
    
    // Public wrapper that simply calls compute.
    pub fn safe_compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        self.compute(numerator, denominator)
    }
}

fn main() {
    let calc = Arc::new(Calculator);
    let calc_ref = Arc::clone(&calc);

    let handle = thread::spawn(move || {
        // The input 0 as divisor is now handled gracefully; an error is returned instead of causing a panic.
        calc_ref.compute(100, 0)
    });

    match handle.join().unwrap() {
        Ok(value) => println!("Computed result: {}", value),
        Err(err) => println!("Encountered error: {}", err),
    }
}