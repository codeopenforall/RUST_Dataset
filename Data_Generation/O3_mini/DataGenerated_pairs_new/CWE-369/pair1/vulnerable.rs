//////////////////////////////////////////////
// Vulnerable Code: Division implementation
//////////////////////////////////////////////
#![allow(dead_code)]
use std::thread;

/// A trait representing a complex arithmetic operation.
trait Arithmetic {
    /// Performs a division in an unsafe context.
    fn operate(&self, numerator: i32, denominator: i32) -> i32;
}

/// A struct implementing the arithmetic operation.
struct Engine;

impl Arithmetic for Engine {
    fn operate(&self, numerator: i32, denominator: i32) -> i32 {
        // UNSAFE: Read the denominator via a raw pointer and perform division
        unsafe {
            // The vulnerability: No check if 'denominator' is zero.
            let ptr = &denominator as *const i32;
            let div = *ptr;
            numerator / div   // <-- Vulnerable division (no zero-check)
        }
    }
}

/// Spawns a thread to perform the division.
fn process(numerator: i32, denominator: i32) -> i32 {
    let engine = Engine;
    // Spawn a new thread to simulate concurrent calculation.
    let handle = thread::spawn(move || {
        engine.operate(numerator, denominator)
    });
    handle.join().unwrap()
}

/// Public API used by the test oracle.
pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    process(numerator, denominator)
}

fn main() {
    // Intentional division by zero (10 / 0) to demonstrate the vulnerability.
    // This will lead to a panic at runtime.
    let result = process(10, 0);
    println!("Calculation result: {}", result);
}