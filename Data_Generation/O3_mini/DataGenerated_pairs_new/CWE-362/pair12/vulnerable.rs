////////////////////////////////////////////////////////////
// Vulnerable Code for Race Condition (CWE-362)
// This code uses unsynchronized modifications to a shared global counter 
// using an unsafe mutable static variable and a manual read-modify-write
// sequence. The race condition arises because multiple threads read, yield,
// and then write back to the same variable without atomicity or synchronization.
////////////////////////////////////////////////////////////
use std::thread;

static mut GLOBAL: u32 = 0;

fn run() -> u32 {
    // Reset the global counter (unsynchronized)
    unsafe {
        GLOBAL = 0;
    }
    // Spawn several threads that concurrently increment the shared counter.
    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                // Intentionally non-atomic update: read, yield, and then write.
                unsafe {
                    let temp = GLOBAL;          // Read current value (line 8)
                    let new_val = temp.wrapping_add(1); // Compute new value (line 9)
                    thread::yield_now();        // Force a context switch (line 10)
                    GLOBAL = new_val;           // Write back the new value (line 11)
                }
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    // Return the final value of the counter.
    unsafe { GLOBAL }
}

fn main() {
    let result = run();
    println!("Final counter value: {}", result);
}