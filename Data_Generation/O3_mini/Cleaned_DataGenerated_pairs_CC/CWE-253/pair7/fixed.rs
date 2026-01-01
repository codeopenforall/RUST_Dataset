#!/usr/bin/env rust
use std::sync::{Arc, Mutex};
use std::thread;
fn calc(v: u32) -> Result<u32, &'static str> {
    let arr = [10, 20, 30, 40, 50];
    unsafe {
        let ptr = arr.as_ptr();
        let _ = *ptr; 
    }
    if v == 0 {
        Err("division by zero")
    } else {
        Ok(100 / v)
    }
}
fn process(v: u32) -> Result<u32, &'static str> {
    calc(v)
}
fn main() {
    let input = 0u32;
    let data = Arc::new(Mutex::new(input));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let val = *data_clone.lock().unwrap();
        process(val)
    });
    match handle.join().unwrap() {
        Ok(result) => println!("Operation succeeded: result = {}", result),
        Err(err) => println!("Operation failed: error = {}", err),
    }
}
