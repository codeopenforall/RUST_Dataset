/* Fixed code for improper check for exceptional conditions (CWE-754) */
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
    // Unsafe memory allocation & deallocation still used, but error handling is improved.
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
        Err(TaskError::Timeout) => Err("Operation timed out"),
        Err(TaskError::Canceled) => Err("Operation canceled"),
        Err(_) => Err("Unknown error occurred"),
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