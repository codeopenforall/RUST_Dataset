#!/usr/bin/env rust
// Fixed code sample for CWE-253: Correctly handling function return values
use std::sync::{Arc, Mutex};
use std::thread;

fn calc(v: u32) -> Result<u32, &'static str> {
    // Dummy array to simulate unsafe pointer manipulation.
    let arr = [10, 20, 30, 40, 50];
    unsafe {
        // Using an unsafe block for pointer arithmetic.
        let ptr = arr.as_ptr();
        let _ = *ptr; // Read first element (dummy operation)
    }
    // If the input is zero, return an error.
    if v == 0 {
        Err("division by zero")
    } else {
        Ok(100 / v)
    }
}

fn process(v: u32) -> Result<u32, &'static str> {
    // CORRECT: Propagate the error result to the caller without misinterpretation.
    calc(v)
}

fn main() {
    // Test input that triggers the error condition (division by zero).
    let input = 0u32;
    let data = Arc::new(Mutex::new(input));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let val = *data_clone.lock().unwrap();
        process(val)
    });
    // The caller now correctly interprets the error condition.
    match handle.join().unwrap() {
        Ok(result) => println!("Operation succeeded: result = {}", result),
        Err(err) => println!("Operation failed: error = {}", err),
    }
}