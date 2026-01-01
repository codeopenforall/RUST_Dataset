#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Operator {
    multiplier: u32,
}

impl Operator {
    // Correctly computes the product by promoting operands to u64 before multiplication.
    fn compute(&self, input: u32) -> u64 {
        let a = input as u64;
        let b = self.multiplier as u64;
        a * b
    }
}

fn main() {
    // Using the same multiplier: 100,000 and input: 50,000.
    let op = Arc::new(Operator { multiplier: 100_000 });
    let test_input: u32 = 50_000;
    let op_clone = Arc::clone(&op);

    // Spawn a thread to simulate concurrent access.
    let handle = thread::spawn(move || {
        op_clone.compute(test_input)
    });
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
    // The fixed code correctly computes 5,000,000,000 by using 64-bit arithmetic.
    if result != 5_000_000_000 {
        std::process::exit(1);
    }
    std::process::exit(0);
}