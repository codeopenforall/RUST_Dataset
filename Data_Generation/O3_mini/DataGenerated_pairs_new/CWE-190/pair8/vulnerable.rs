use std::sync::Arc;
use std::thread;

struct Processor {
    factor: u32,
}

impl Processor {
    // Performs multiplication without checking for overflow.
    // If the product exceeds u32::MAX, it silently wraps around.
    fn calculate(&self, input: u32) -> u32 {
        // Unsafe block bypasses Rust’s built-in overflow checks.
        unsafe {
            let prod: u64 = (input as u64).wrapping_mul(self.factor as u64);
            // Vulnerability: conversion from u64 to u32 is done without checking,
            // causing wrap-around when prod > u32::MAX.
            prod as u32
        }
    }

    // This wrapper returns a Result type but simply embeds the unchecked result.
    fn check(&self, input: u32) -> Result<u32, &'static str> {
        Ok(self.calculate(input))
    }
}

fn main() {
    // Create a shared instance that will be accessed concurrently.
    let processor = Arc::new(Processor { factor: 70000 });
    let input = 70000; // Triggering input that makes product exceed u32::MAX

    // Spawn multiple threads to simulate concurrent usage.
    let mut handles = Vec::new();
    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        handles.push(thread::spawn(move || {
            let result = proc_clone.check(input);
            // All threads print the (wrapped) computed result.
            println!("Result: {:?}", result);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}