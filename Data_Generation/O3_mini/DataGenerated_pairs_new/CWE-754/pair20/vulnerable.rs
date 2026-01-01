/* Vulnerable code for improper check for exceptional conditions (CWE-754) */
#![allow(dead_code)]
use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug, PartialEq)]
enum TaskError {
    Timeout,
    Canceled,
    Other,
}

fn run_task(input: i32) -> Result<i32, TaskError> {
    // Simulate an error when input is zero
    if input == 0 {
        return Err(TaskError::Timeout);
    }
    let layout = Layout::new::<i32>();
    // Unsafe memory allocation & deallocation mimicking complex real-world usage.
    let ptr = unsafe { alloc(layout) as *mut i32 };
    if ptr.is_null() {
        panic!("Allocation failed");
    }
    unsafe {
        *ptr = input * 2;
    }
    let result = unsafe { *ptr };
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
    Ok(result)
}

fn process(input: i32) -> Result<i32, &'static str> {
    match run_task(input) {
        Ok(v) => Ok(v),
        Err(e) => {
            // Vulnerability: improper check for error conditions.
            // Instead of handling timeout correctly, any error not equal to 'Canceled'
            // is mistakenly treated as a successful outcome (defaulting to 0).
            if e != TaskError::Canceled {
                // [VULNERABLE] Erroneously converting a timeout (or other) error into Ok(0)
                Ok(0)
            } else {
                Err("Operation canceled")
            }
        }
    }
}

fn main() {
    // Triggering input that should cause an error (input = 0)
    let result = process(0);
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}