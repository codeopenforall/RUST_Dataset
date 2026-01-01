//////////////////////////////////////////////
// File: src/bin/fixed.rs
//////////////////////////////////////////////
use std::{
    sync::{Arc, Mutex},
    thread,
};

/// Safely performs multiplication by using 64-bit arithmetic and checked operations.
/// This prevents integer overflow when the true product exceeds u32 capacity.
fn calculate(a: u32, b: u32) -> u64 {
    // Cast operands to u64 to perform safe multiplication.
    // checked_mul returns an Option ensuring that an overflow would cause a graceful error.
    (a as u64)
        .checked_mul(b as u64)
        .expect("Multiplication overflow detected")
}

fn main() {
    let operand1: u32 = 100_000;
    let operand2: u32 = 50_000;
    
    // Shared state with Arc+Mutex, similar to the vulnerable version.
    let shared_result = Arc::new(Mutex::new(0u64));
    let shared_result_clone = Arc::clone(&shared_result);

    // Spawn a thread to perform the calculation.
    let handle = thread::spawn(move || {
        let result = calculate(operand1, operand2);
        let mut lock = shared_result_clone.lock().unwrap();
        *lock = result;
    });
    handle.join().unwrap();
    
    let final_value = *shared_result.lock().unwrap();
    // The correct multiplication result (5,000,000,000) is produced by avoiding the overflow.
    println!("Result: {}", final_value);
}