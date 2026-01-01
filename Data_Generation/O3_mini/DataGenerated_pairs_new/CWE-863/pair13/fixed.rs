/* Corrected Version - This version fixes the authorization check by using 
   proper string comparison. It enforces resource allocation limits uniformly for unauthorized users,
   thus mitigating the risk of uncontrolled resource consumption. */
  
#![allow(unused)]
use std::thread;

const MAX_LIMIT: usize = 1024 * 1024; // 1 MB limit for non-privileged requests

// This function now correctly compares the user identifier and enforces the resource cap for unauthorized users.
fn process_request(user: &str, amount: usize) -> Result<(), String> {
    // Proper authorization check: string equality comparison.
    // Privileged user "admin" is allowed to request larger amounts, while others are capped.
    if user != "admin" && amount > MAX_LIMIT {
        return Err("Unauthorized request exceeds the permitted limit".to_string());
    }
    // Allocate the requested memory safely.
    let mut data = Vec::with_capacity(amount);
    data.resize(amount, 0);
    Ok(())
}

fn run_app() {
    // Simulate concurrent requests.
    let mut handles = Vec::new();
    let requests = vec![
        ("guest", 1024),          // Normal small allocation - should succeed.
        ("admin", 2 * 1024 * 1024), // Admin allowed larger allocation - should succeed.
        ("guest", 2 * 1024 * 1024), // Unauthorized huge allocation - now correctly rejected.
    ];

    for (user, size) in requests {
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