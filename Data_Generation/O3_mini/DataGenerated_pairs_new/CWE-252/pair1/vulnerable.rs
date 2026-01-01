//////////////////////////////////////////
// Vulnerable Code Sample
//////////////////////////////////////////
use std::thread;

// Global mutable state accessed via an unsafe block.
// In production, such globals would be wrapped in proper synchronization.
static mut GLOBAL: u32 = 0;

/// Simulates an unsafe external update (e.g., via an FFI call).
/// Returns an error if the provided value is considered invalid.
unsafe fn unsafe_update(value: u32) -> Result<(), &'static str> {
    // Simulated check: values > 1000 are considered a failure.
    if value > 1000 {
        Err("value too high")
    } else {
        Ok(())
    }
}

/// Performs an update using an unsafe call.
/// The return value of unsafe_update is ignored.
/// On error, the code still proceeds to update the global state,
/// which is a CWE-252 unchecked return value vulnerability.
fn execute() -> u32 {
    let value: u32 = 1500;
    unsafe {
        // Vulnerability: Ignoring the Result from unsafe_update.
        let _ = unsafe_update(value);
        // Proceed to update GLOBAL regardless of the call's outcome.
        GLOBAL = GLOBAL.wrapping_add(value);
        GLOBAL
    }
}

fn main() {
    // Spawn a thread to add some concurrency complexity.
    let handle = thread::spawn(|| {
        let res = execute();
        println!("Result in thread: {}", res);
    });
    handle.join().unwrap();
}