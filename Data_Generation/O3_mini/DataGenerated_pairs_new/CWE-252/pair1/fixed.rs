//////////////////////////////////////////
// Corrected Code Sample
//////////////////////////////////////////
use std::thread;

// Global mutable state accessed via an unsafe block.
// In production, proper synchronization (e.g., Mutex) should be used.
static mut GLOBAL: u32 = 0;

/// Simulates an unsafe external update (e.g., an FFI function).
/// Returns an error if the provided value is considered invalid.
unsafe fn unsafe_update(value: u32) -> Result<(), &'static str> {
    if value > 1000 {
        Err("value too high")
    } else {
        Ok(())
    }
}

/// Performs an update using an unsafe call.
/// The return value from unsafe_update is properly examined.
/// On an error, the global state is not updated.
fn execute() -> u32 {
    let value: u32 = 1500;
    unsafe {
        // Properly check the result returned by unsafe_update.
        match unsafe_update(value) {
            Ok(()) => {
                GLOBAL = GLOBAL.wrapping_add(value);
            },
            Err(e) => {
                // Handle the error appropriately; here, we simply log it.
                println!("Update failed: {}", e);
            }
        }
        GLOBAL
    }
}

fn main() {
    // Spawn a thread to add concurrency complexity.
    let handle = thread::spawn(|| {
        let res = execute();
        println!("Result in thread: {}", res);
    });
    handle.join().unwrap();
}