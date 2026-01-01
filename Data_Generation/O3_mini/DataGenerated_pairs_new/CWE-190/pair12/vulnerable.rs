/* Vulnerable Code */
use std::sync::Arc;
use std::thread;

struct Engine;

trait Operations {
    // Using an unsafe method that performs multiplication and then casts the result into a smaller type.
    // This cast truncates the high bits if the product exceeds u32 capacity.
    unsafe fn compute(&self, a: u64, b: u64) -> u32;
}

impl Operations for Engine {
    unsafe fn compute(&self, a: u64, b: u64) -> u32 {
        // Line 12: Multiplying without checking for overflow.
        let product = a * b;
        // Line 14: UNSAFE CAST: Casting product (u64) to u32 produces truncation if product > u32::MAX.
        product as u32
    }
}

fn run() -> u64 {
    let engine = Arc::new(Engine{});
    let a: u64 = 300_000;
    let b: u64 = 30_000; // Expected product: 9,000,000,000 (> u32::MAX)
    let engine_clone = engine.clone();
    let handler = thread::spawn(move || {
        // Unsafe code region performing unchecked conversion.
        unsafe { engine_clone.compute(a, b) }
    });
    let result_u32 = handler.join().unwrap();
    // Convert to u64 to facilitate validation in the test.
    result_u32 as u64
}

fn main() {
    let result = run();
    println!("Computed result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn check_result() {
        // The expected correct product is 9,000,000,000.
        // The vulnerable code produces a truncated result (due to cast), so this test should fail.
        assert_eq!(run(), 9_000_000_000, "Computation did not yield the expected result");
    }
}