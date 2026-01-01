///////////////////////////////
// Fixed Code Example
///////////////////////////////
use std::thread;

/// Safely performs an integer division by first validating the denominator.
/// Returns an Err variant if the denominator is zero, thus preventing a division by zero.
pub fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero avoided")
    } else {
        // Even though the operation is still done in an unsafe block,
        // the check ensures that the divisor is never zero.
        unsafe {
            Ok(a / b)
        }
    }
}

fn main() {
    // Use a safe divisor to avoid triggering division by zero.
    let res = compute(50, 1);
    match res {
        Ok(val) => println!("Computation result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    // Spawn a thread to simulate concurrent execution with safe input.
    let handle = thread::spawn(|| {
        let res_thread = compute(100, 1);
        match res_thread {
            Ok(val) => println!("Thread result: {}", val),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    let _ = handle.join();
}