//////////////////////////////////////////////
// File: src/bin/vulnerable.rs
//////////////////////////////////////////////
use std::{
    sync::{Arc, Mutex},
    thread,
};

/// Performs multiplication in a way that is vulnerable to integer overflow.
/// The computation is done in an unsafe block using raw pointer manipulation.
fn calculate(a: u32, b: u32) -> u64 {
    // The multiplication uses wrapping_mul without detection of overflow.
    let intermediate = a.wrapping_mul(b);
    // Mimic unsafe operations on the computed result.
    let mut value = intermediate;
    unsafe {
        let ptr: *mut u32 = &mut value as *mut u32;
        // Directly write to the memory location, simulating unsafe manipulation.
        *ptr = intermediate;
    }
    value as u64  // Convert to u64, but the value is already truncated.
}

fn main() {
    // Use operands where the true mathematical result (5_000_000_000)
    // exceeds u32::MAX (4_294_967_295), causing an integer wraparound.
    let operand1: u32 = 100_000;
    let operand2: u32 = 50_000;
    
    // Use an Arc+Mutex to simulate shared state accessed concurrently.
    let shared_result = Arc::new(Mutex::new(0u64));
    let shared_result_clone = Arc::clone(&shared_result);

    // Spawn a thread to simulate concurrent computation.
    let handle = thread::spawn(move || {
        let result = calculate(operand1, operand2);
        // Use an unsafe block to simulate direct memory handling,
        // although here it's only used to update the shared state.
        unsafe {
            let mut lock = shared_result_clone.lock().unwrap();
            *lock = result;
        }
    });
    handle.join().unwrap();
    
    let final_value = *shared_result.lock().unwrap();
    // Output final result; due to overflow the printed result is incorrect.
    println!("Result: {}", final_value);
}