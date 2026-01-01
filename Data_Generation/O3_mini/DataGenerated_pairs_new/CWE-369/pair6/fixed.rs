//////////////////////////////
// Fixed Version
//////////////////////////////
use std::io;
use std::thread;
use std::sync::mpsc;

// A trait defining arithmetic operations.
trait Operations {
    // Performs an integer division using an unsafe raw pointer to the denominator.
    // Returns a Result wrapping the division result or an error message if the denominator is zero.
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str>;
}

struct Arithmetic;

impl Operations for Arithmetic {
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str> {
        // Fix: Check if the value pointed to is zero before performing the division.
        let denominator = *denominator_ptr;
        if denominator == 0 {
            return Err("Division by zero");
        }
        Ok(numerator / denominator)
    }
}

fn main() {
    // Allocate an integer on the heap.
    let boxed = Box::new(0i32);
    // Obtain a mutable raw pointer to the allocated integer.
    let mut raw_ptr: *mut i32 = Box::into_raw(boxed);

    // Use a channel to obtain user input concurrently.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let value: i32 = input.trim().parse().expect("Invalid integer input");
        tx.send(value).expect("Channel send failed");
    });
    let value = rx.recv().expect("Channel receive failed");

    // Update the boxed value unsafely.
    unsafe {
        *raw_ptr = value;
    }

    let calc = Arithmetic;
    // Call the fixed unsafe division method.
    let result = unsafe { calc.div_safe(100, raw_ptr as *const i32) };
    match result {
        Ok(res) => println!("Result: {}", res),
        Err(e) => println!("Error: {}", e),
    }
    
    // Clean up the allocated memory.
    unsafe {
        Box::from_raw(raw_ptr);
    }
}