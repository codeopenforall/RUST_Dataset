////////////////////////////////////////////////////////////
// Fixed Code Sample addressing CWE-203
// This version neutralizes the information leak by using a
// constant-time password comparison and always returning a generic
// error message for any authentication failure.
////////////////////////////////////////////////////////////
#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

// Unsafe helper function that performs a constant-time comparison.
// It always iterates through the maximum length of the provided inputs
// to avoid timing discrepancies.
unsafe fn constant_time_compare(stored: &str, provided: &str) -> bool {
    let n = stored.len().max(provided.len());
    let mut result: u8 = 0;
    for i in 0..n {
        let sc = if i < stored.len() { *stored.as_ptr().add(i) } else { 0 };
        let pc = if i < provided.len() { *provided.as_ptr().add(i) } else { 0 };
        result |= sc ^ pc;
    }
    // Only if lengths are equal and all bytes match will result be zero.
    (result == 0) && (stored.len() == provided.len())
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

    // The authenticate method in this revision always returns the same error
    // message ("Invalid credentials") regardless of whether the username is
    // found or the password comparison fails.
    // It also uses a constant-time password comparison function.
    pub fn authenticate(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let stored = self.accounts.get(username).map(|s| s.as_str()).unwrap_or("");
        if !stored.is_empty() && unsafe { constant_time_compare(stored, password) } {
            Ok(())
        } else {
            // Fixed: Generic error message to prevent enumeration.
            Err("Invalid credentials")
        }
    }
}

fn main() {
    let auth = AccessControl::new();
    let auth_arc = Arc::new(auth);
    let cloned = Arc::clone(&auth_arc);
    let handle = thread::spawn(move || {
        // Attempt authentication with a known username but wrong password.
        match cloned.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Authenticated"),
            Err(e) => println!("Error: {}", e)
        }
    });
    handle.join().unwrap();
}