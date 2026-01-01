//////////////////////////////
// Fixed version: Division-by-Zero Check Added
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Calculator;

impl Calculator {
    // The fixed compute method validates the divisor before performing the division,
    // thereby preventing a division-by-zero error.
    fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        // No unsafe block is required since there's no unchecked arithmetic.
        Ok(a / b)
    }
}

fn main() {
    let calc = Arc::new(Calculator);
    let calc_clone = Arc::clone(&calc);
    let join_handle = thread::spawn(move || {
        // Even when using zero as the divisor, the check prevents a panic.
        calc_clone.compute(100, 0)
    });
    let res = join_handle.join().expect("Thread panicked unexpectedly.");
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}