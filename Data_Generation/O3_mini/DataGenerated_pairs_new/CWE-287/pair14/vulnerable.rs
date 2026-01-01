///////////////////////////////////////////////////////
// Vulnerable Code: Improper token verification using //
// unsafe memory access leads to authentication bypass //
///////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    valid: String,
}

impl Authenticator {
    // This method compares only the prefix of the valid token,
    // allowing an attacker to supply a shorter token that matches.
    fn check(&self, token: &str) -> bool {
        // Improper token check using unsafe operations:
        unsafe {
            let valid_bytes = self.valid.as_bytes();
            let token_bytes = token.as_bytes();
            let len = token_bytes.len();
            // If provided token is longer than valid token, reject
            if len > valid_bytes.len() {
                return false;
            }
            // Only compare the first len bytes; if token is a prefix,
            // the check incorrectly passes.
            let p_valid = valid_bytes.as_ptr();
            // Construct a slice with the same length as the provided token.
            let slice_valid = std::slice::from_raw_parts(p_valid, len);
            // Flawed: This compares only the prefix of the valid token.
            slice_valid == token_bytes
        }
    }
}

fn main() {
    // Initialize the authenticator with the expected token.
    let auth = Arc::new(Mutex::new(Authenticator {
        valid: "supersecrettoken".to_string(),
    }));
    let auth_clone = Arc::clone(&auth);

    // Spawn a thread to simulate a concurrent authentication request.
    let handle = thread::spawn(move || {
        let locked = auth_clone.lock().unwrap();
        // The supplied token "supersecret" is only a prefix,
        // but due to the flaw, it mistakenly passes authentication.
        println!("Authentication result: {}", locked.check("supersecret"));
    });
    handle.join().unwrap();
}