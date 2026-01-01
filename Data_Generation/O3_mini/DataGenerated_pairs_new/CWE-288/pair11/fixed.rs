//////////////////////////////
// Fixed Code Example       //
// CWE-288: Authentication Bypass Mitigation
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    secret: String,
    debug: bool,
}

impl Authenticator {
    fn new(secret: &str, debug: bool) -> Self {
        Self {
            secret: secret.to_string(),
            debug,
        }
    }

    fn verify(&self, provided: &str) -> bool {
        // Even if debug mode is active, the authentication check must be enforced.
        if self.debug {
            // Optionally log that debug mode is active without bypassing auth.
            println!("Debug mode active: Enforcing authentication.");
        }
        self.secret == provided
    }
}

fn main() {
    // Shared state created with Arc<Mutex<_>> for a concurrent scenario.
    // Debug mode is still enabled; however, it no longer bypasses the authentication logic.
    let auth = Arc::new(Mutex::new(Authenticator::new("securepassword", true)));
    let auth_clone = Arc::clone(&auth);

    let handler = thread::spawn(move || {
        let lock = auth_clone.lock().unwrap();
        // The authentication now correctly compares the provided credential.
        if lock.verify("badpass") {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
    });
    handler.join().unwrap();
}