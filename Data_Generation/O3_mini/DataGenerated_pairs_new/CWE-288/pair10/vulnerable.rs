///////////////////////////////////////////////////////////////
// Vulnerable Code for an Authentication Bypass Issue (CWE-288)
///////////////////////////////////////////////////////////////
use std::env;
use std::thread;
use std::sync::Arc;

struct AuthManager {
    bypass: bool,
}

impl AuthManager {
    pub fn new() -> Self {
        let mut bypass = false;
        // POTENTIAL SECURITY ISSUE:
        // An unsafe block is used to check an environment variable.
        // When the environment variable "DEBUG_MODE" is set to "1",
        // the bypass flag is set via an unsafe pointer write,
        // allowing authentication to be skipped.
        unsafe {
            if let Ok(debug) = env::var("DEBUG_MODE") {
                if debug == "1" {
                    // UNSAFE POINTER MANIPULATION that bypasses security check
                    let bypass_ptr: *mut bool = &mut bypass;
                    *bypass_ptr = true;
                }
            }
        }
        AuthManager { bypass }
    }
    
    pub fn check(&self, username: &str, password: &str) -> bool {
        // SECURITY FLAW: When bypass is true, credentials are not verified.
        if self.bypass {
            return true;
        }
        // Normal authentication flow (for example purposes).
        username == "admin" && password == "secret"
    }
}

fn main() {
    // Concurrency simulates a production-like environment.
    let manager = Arc::new(AuthManager::new());
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        // Simulated authentication request with invalid credentials.
        let result = manager_clone.check("user", "wrongpass");
        println!("Authenticated: {}", result);
    });
    handle.join().unwrap();
}