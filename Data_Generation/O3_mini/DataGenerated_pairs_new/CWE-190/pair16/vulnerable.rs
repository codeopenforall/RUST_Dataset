#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Operator {
    multiplier: u32,
}

impl Operator {
    // Unsafe method that performs multiplication in a 32‐bit domain.
    // The multiplication is done using u32 arithmetic, so if the mathematical
    // product exceeds u32::MAX it will wrap around before being cast to u64.
    unsafe fn compute(&self, input: u32) -> u64 {
        // Vulnerability: Performing multiplication with potential overflow in u32.
        let product = input.wrapping_mul(self.multiplier);
        product as u64
    }
}

fn main() {
    // Set a large multiplier so that for the test input the mathematically correct product
    // 50,000 * 100,000 = 5,000,000,000 exceeds the capacity of u32.
    let op = Arc::new(Operator { multiplier: 100_000 });
    let test_input: u32 = 50_000; // This value triggers the overflow vulnerability.
    let op_clone = Arc::clone(&op);

    // Use a separate thread to simulate concurrent usage.
    let handle = thread::spawn(move || {
        // Unsafe block invocation of the compute method.
        unsafe { op_clone.compute(test_input) }
    });
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
    // The intended correct product is 5,000,000,000.
    // However, due to the u32 arithmetic overflow the returned value is incorrect.
    if result != 5_000_000_000 {
        // Signal failure if the incorrect (wrapped) value is detected.
        std::process::exit(1);
    }
    std::process::exit(0);
}