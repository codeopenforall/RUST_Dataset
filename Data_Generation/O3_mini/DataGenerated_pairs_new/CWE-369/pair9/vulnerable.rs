///////////////////////////////
// Vulnerable Code Example
///////////////////////////////
use std::thread;

/// Performs an integer division using an unsafe block without checking if the denominator is zero.
/// This function is intended to return a Result, but if the denominator is zero the unchecked division will panic.
pub fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    unsafe {
        // Division without validation – CWE-369 vulnerability: unchecked division by zero.
        let result = a / b;
        Ok(result)
    }
}

fn main() {
    // This intentionally triggers division by zero.
    let res = compute(50, 0);
    match res {
        Ok(val) => println!("Computation result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    // Spawn a thread to simulate concurrent execution that also triggers the vulnerability.
    let handle = thread::spawn(|| {
        let res_thread = compute(100, 0);
        match res_thread {
            Ok(val) => println!("Thread result: {}", val),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    let _ = handle.join();
}