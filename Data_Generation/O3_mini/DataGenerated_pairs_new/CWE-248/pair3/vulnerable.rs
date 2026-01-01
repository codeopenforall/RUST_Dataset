///////////////////////// Vulnerable Version /////////////////////////
// This example demonstrates an unhandled panic issue in a multithreaded context.
// In this version, when an abnormal input (negative integer) is provided,
// the unsafe operation panics and the panic propagates (via unwrap() on thread join),
// causing abrupt termination. This reflects CWE-248 where the program does not catch
// or handle the panic properly.

use std::thread;

unsafe fn compute(val: i32) -> i32 {
    // This unsafe function panics when receiving a negative value
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    // Simulate some nontrivial computation
    val * 2
}

fn run_task(input: i32) -> Result<i32, String> {
    // Spawn a thread to perform the computation unsafely.
    let handle = thread::spawn(move || {
        // Unsafe block to mimic complex real-world code interacting with low-level APIs.
        unsafe { compute(input) }
    });
    // The thread join is unwrapped without any panic catching; if a panic happens,
    // it propagates to the main thread, leading to abnormal termination.
    Ok(handle.join().unwrap())
}

fn main() {
    // Trigger abnormal termination by passing a negative value.
    // When running this code, the program will panic and exit.
    let _ = run_task(-1);
}