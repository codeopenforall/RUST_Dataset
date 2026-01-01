//////////////////////////////
// Vulnerable Code Example  //
// CWE-288: Authentication Bypass Using an Alternate Path (Debug Bypass)
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
        // In debug mode, an unsafe alternate path bypasses the authentication check
        if self.debug {
            // SAFETY: The following unsafe block calls a function that unconditionally
            // returns true. This bypasses the intended secret-check logic and presents
            // an authentication bypass.
            if unsafe { self.alternate_path() } {
                return true;
            }
        }
        self.secret == provided
    }

    unsafe fn alternate_path(&self) -> bool {
        // The unsafe code returns true regardless of any checks.
        // This is the exploited authentication bypass path.
        true
    }
}

fn main() {
    // Simulate shared state using Arc<Mutex<_>> in a concurrent scenario.
    // The debug flag is enabled; thus, the bypass path is active.
    let auth = Arc::new(Mutex::new(Authenticator::new("securepassword", true)));
    let auth_clone = Arc::clone(&auth);

    // Spawn a thread that attempts to verify credentials.
    let handler = thread::spawn(move || {
        let lock = auth_clone.lock().unwrap();
        // Here, even when given a wrong credential ("badpass"), the unsafe bypass will cause
        // the verification to mistakenly succeed in debug mode.
        if lock.verify("badpass") {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
    });
    handler.join().unwrap();
}