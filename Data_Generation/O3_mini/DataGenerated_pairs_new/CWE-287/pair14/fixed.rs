/////////////////////////////////////////////////////////////
// Corrected Code: Proper and full token verification logic //
/////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    valid: String,
}

impl Authenticator {
    // Correct full equality check to ensure proper authentication.
    fn check(&self, token: &str) -> bool {
        // Direct and safe comparison of the complete token.
        token == self.valid
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
        // The token must exactly match to pass; "supersecret" will not be accepted.
        println!("Authentication result: {}", locked.check("supersecrettoken"));
    });
    handle.join().unwrap();
}