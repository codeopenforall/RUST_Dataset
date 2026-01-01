/* Vulnerable Version - This example demonstrates an authorization check vulnerability 
   where an unsafe pointer comparison is used. This leads to incorrect authorization 
   checking, permitting unbounded resource allocations from unauthorized users.
   Additionally, the unchecked allocation of memory may exhaust resources. */
  
#![allow(unused)]
use std::thread;

const MAX_LIMIT: usize = 1024 * 1024; // 1 MB limit for authorized requests

// This function improperly checks if the caller is privileged by comparing raw pointers.
// If the caller is not exactly the "admin" literal in memory, no allocation cap is enforced.
fn process_request(user: &str, amount: usize) -> Result<(), String> {
    unsafe {
        // Vulnerable comparison: comparing raw pointers of the input and the literal.
        // This is unsafe because identical string contents from different allocations will not match.
        if std::ptr::eq(user.as_ptr(), "admin".as_ptr()) {
            // For "admin", enforce the resource cap.
            if amount > MAX_LIMIT {
                return Err("Authorized request exceeds the permitted limit".to_string());
            }
        } else {
            // For non-privileged users, the allocation cap is mistakenly omitted.
            // The intended design was to restrict these requests, but due to the bug,
            // an attacker could trigger large memory allocations.
        }
    }
    // Allocate a vector with the requested capacity (potentially huge).
    // Using resize to simulate resource initialization.
    let mut data = Vec::with_capacity(amount);
    data.resize(amount, 0);
    Ok(())
}

fn run_app() {
    // Simulate concurrent requests.
    let mut handles = Vec::new();
    let requests = vec![
        ("guest", 1024),          // Normal small allocation - should succeed.
        ("admin", 512 * 1024),      // For admin, within limit - should succeed.
        ("guest", 2 * 1024 * 1024), // Unauthorized huge allocation - vulnerability: not rejected.
    ];

    for (user, size) in requests {
        // Spawn a thread for each request.
        let user = user.to_string();
        handles.push(thread::spawn(move || {
            match process_request(&user, size) {
                Ok(_) => println!("Request from '{}' with {} bytes succeeded.", user, size),
                Err(e) => println!("Request from '{}' with {} bytes failed: {}", user, size, e),
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    run_app();
}