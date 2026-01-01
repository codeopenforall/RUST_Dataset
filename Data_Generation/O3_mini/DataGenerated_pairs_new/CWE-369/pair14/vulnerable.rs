/* 
   This Rust program demonstrates a calculation service that performs division inside an unsafe block.
   It spawns a thread to compute the result of dividing a given numerator by a user-supplied denominator.
   The vulnerability lies in the unchecked division: if the denominator is zero, the unsafe block
   directly dereferences the pointer to perform division, causing a divide-by-zero error at runtime.
*/
use std::sync::Arc;
use std::thread;

struct Calculator {}

impl Calculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(&self, a: i32, b: i32) -> i32 {
        unsafe {
            let ptr: *const i32 = &b as *const i32;
            let divisor = *ptr;
            // Vulnerability: division without checking if 'divisor' == 0.
            a / divisor
        }
    }
}

fn main() {
    let calc = Calculator::new();
    // For demonstration, using 0 as the denominator to trigger the vulnerability.
    let arc_calc = Arc::new(calc);
    let arc_clone = Arc::clone(&arc_calc);
    let handle = thread::spawn(move || {
        arc_clone.compute(10, 0)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}