//////////////////////////////////////////////////////////////
// This program simulates a resource allocation module that
// spawns threads and performs unsafe memory writes.
// It erroneously allows unbounded resource allocation and lacks
// access control checks on user roles. An attacker supplying
// an unprivileged role can force the program to allocate arbitrarily
// many threads, possibly leading to a denial‐of‐service.
//////////////////////////////////////////////////////////////

use std::thread;

pub struct Manager;

impl Manager {
    // This function allocates resources by spawning threads.
    // It does not enforce limits on the number of allocated threads
    // and does not perform any role-based authorization.
    pub fn allocate_resources(role: &str, count: usize) -> Result<usize, String> {
        // Unsafe block performing a dummy memory write.
        // (Simulates low-level operations found in real systems.)
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            // No check on role; all users allowed.
            let handle = thread::spawn(|| {
                unsafe {
                    // Unsafe manual memory write.
                    let mut val: i32 = 0;
                    let ptr: *mut i32 = &mut val as *mut i32;
                    *ptr = 42;
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete.
        for handle in handles {
            let _ = handle.join();
        }

        // Always returns success; no authorization error.
        Ok(count)
    }
}

fn main() {
    // For demonstration, allocate resources for a non-admin user.
    // An attacker could supply a huge count to exhaust system resources.
    match Manager::allocate_resources("user", 1000) {
        Ok(num) => println!("Allocated {} resources successfully.", num),
        Err(err) => eprintln!("Error: {}", err),
    }
}