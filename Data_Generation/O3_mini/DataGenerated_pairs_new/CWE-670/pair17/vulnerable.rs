//////////////////////////////////////////////
/// A complex application that computes a value
/// based on a global function pointer. It spawns a thread
/// that unsafely updates the global pointer without any
/// synchronization or integrity checks. Due to this, the
/// control flow intended to use the approved computation
/// (doubling the input) is hijacked to use an alternative that
/// multiplies the input by 10. This flawed branch logic violates
/// control flow integrity (CWE-670).
//////////////////////////////////////////////
use std::thread;
use std::time::Duration;

// Global mutable pointer to the active computation function.
// Initially set to the approved function.
static mut ACTION: fn(i32) -> i32 = approved;

// Approved computation: doubles the input.
fn approved(x: i32) -> i32 {
    x * 2
}

// Alternative (malicious) computation: multiplies input by 10.
fn diverted(x: i32) -> i32 {
    x * 10
}

// run_app executes the following sequence:
// 1. Spawns a thread that unsafely modifies the global pointer
//    to point to the diverted function.
// 2. Joins the thread (ensuring the update occurs).
// 3. Calls the global function pointer through an unsafe block.
fn run_app() -> i32 {
    let handler = thread::spawn(|| {
        // UNSAFE: Improper control flow update (vulnerability)
        unsafe {
            ACTION = diverted;
        }
    });
    // Wait for the thread to complete the update.
    let _ = handler.join();
    // Using the global pointer without verifying its integrity.
    let result = unsafe { ACTION(5) };
    result
}

fn main() {
    let res = run_app();
    println!("Computed value: {}", res);
}