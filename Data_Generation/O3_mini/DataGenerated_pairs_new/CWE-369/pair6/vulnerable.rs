//////////////////////////////
// Vulnerable Version
//////////////////////////////
use std::io;
use std::thread;
use std::sync::mpsc;

// A trait defining arithmetic operations.
trait Operations {
    // Performs an integer division using an unsafe raw pointer to the denominator.
    // Returns a Result wrapping the division result.
    // In this version, no check is performed for a zero divisor.
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str>;
}

struct Arithmetic;

impl Operations for Arithmetic {
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str> {
        // Vulnerability: Directly dereference the raw pointer and perform division.
        // If the value pointed to is zero, this will trigger a division by zero panic.
        let denominator = *denominator_ptr;                    // (VULNERABLE: raw pointer dereference)
        let result = numerator / denominator;                  // (VULNERABLE: unchecked division)
        Ok(result)
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
    // Call the unsafe division method.
    let result = unsafe { calc.div_safe(100, raw_ptr as *const i32) };
    println!("Result: {:?}", result);

    // Clean up the allocated memory.
    unsafe {
        Box::from_raw(raw_ptr);
    }
}