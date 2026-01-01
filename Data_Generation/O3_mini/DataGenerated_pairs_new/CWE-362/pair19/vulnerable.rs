//////////////////////////////////////////////////////////////
// This code demonstrates a race condition vulnerability (CWE-362)
// where multiple threads concurrently update a shared counter without
// proper synchronization. The unsynchronized read-modify-write is performed
// on a global pointer using unsafe operations. The lack of atomicity leads
// to a time-of-check to time-of-use bug.
//////////////////////////////////////////////////////////////

use std::thread;

static mut GLOBAL_PTR: *mut u32 = std::ptr::null_mut();

/// Executes a concurrent increment routine without synchronization.
/// Spawns 10 threads where each thread loops 1000 times to increment a shared counter.
/// Returns the final counter value.
pub fn run() -> u32 {
    // Allocate counter on the heap and store its raw pointer in the global variable.
    let boxed = Box::new(0u32);
    let ptr = Box::into_raw(boxed);
    unsafe {
        GLOBAL_PTR = ptr;
    }

    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    // Vulnerability: unsynchronized read-modify-write sequence.
                    if !GLOBAL_PTR.is_null() {
                        let temp = *GLOBAL_PTR; // Read current value.
                        let new_val = temp.wrapping_add(1); // Compute new value.
                        thread::yield_now(); // Hint to scheduler increases chance of race.
                        *GLOBAL_PTR = new_val; // Write new value.
                    }
                }
            }
        }));
    }

    // Wait for all threads to complete.
    for handle in handles {
        let _ = handle.join();
    }

    // Return the result and release the allocated memory.
    unsafe {
        let res = *GLOBAL_PTR;
        let _ = Box::from_raw(GLOBAL_PTR);
        res
    }
}

fn main() {
    let final_count = run();
    println!("Final counter value: {}", final_count);
}