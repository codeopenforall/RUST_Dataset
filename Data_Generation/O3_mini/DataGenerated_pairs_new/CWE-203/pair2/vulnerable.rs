////////////////////////////////////////////////////////////
// Vulnerable Code Sample targeting CWE-203
// This version leaks sensitive information via discrepant
// error messages when authentication fails.
////////////////////////////////////////////////////////////
#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

// Unsafe helper function to compare two strings in a straightforward way.
// This function does a byte‐by‐byte comparison and is not constant time.
unsafe fn compare_passwords(stored: &str, provided: &str) -> bool {
    if stored.len() != provided.len() {
        return false;
    }
    let stored_ptr = stored.as_ptr();
    let provided_ptr = provided.as_ptr();
    for i in 0..stored.len() {
        // Using unsafe pointer arithmetic to access memory.
        if *stored_ptr.add(i) != *provided_ptr.add(i) {
            return false;
        }
    }
    true
}

pub struct AccessControl {
    accounts: HashMap<String, String>, // username => password
}

impl AccessControl {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // Initialize with one account.
        map.insert("alice".to_string(), "secret".to_string());
        AccessControl { accounts: map }
    }

    // The authenticate method returns distinct error messages
    // depending on whether the user exists or password mismatches.
    // This discrepancy enables an attacker to infer account existence.
    pub fn authenticate(&self, username: &str, password: &str) -> Result<(), &'static str> {
        if let Some(stored) = self.accounts.get(username) {
            if unsafe { compare_passwords(stored, password) } {
                Ok(())
            } else {
                // Vulnerability: reveals that the account exists, but the password is wrong.
                // (Line 27)
                Err("Incorrect password for existing user")
            }
        } else {
            // Vulnerability: distinct error indicates that the user does not exist.
            // (Line 31)
            Err("User not found")
        }
    }
}

fn main() {
    let system = AccessControl::new();
    let system_arc = Arc::new(system);
    let cloned = Arc::clone(&system_arc);
    let handle = thread::spawn(move || {
        // Attempt authentication with a known username but wrong password.
        match cloned.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Authenticated"),
            Err(e) => println!("Error: {}", e)
        }
    });
    handle.join().unwrap();
}