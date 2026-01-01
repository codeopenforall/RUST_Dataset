#![allow(dead_code)]
use std::alloc::{alloc, dealloc, Layout};
#[derive(Debug, PartialEq)]
enum TaskError {
    Timeout,
    Canceled,
    Other,
}
fn run_task(input: i32) -> Result<i32, TaskError> {
    if input == 0 {
        return Err(TaskError::Timeout);
    }
    let layout = Layout::new::<i32>();
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
            if e != TaskError::Canceled {
                Ok(0)
            } else {
                Err("Operation canceled")
            }
        }
    }
}
fn main() {
    let result = process(0);
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}
