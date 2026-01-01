///////////////////////////////////////////////////////////////
// Fixed Code Addressing the Authentication Bypass (CWE-288)
///////////////////////////////////////////////////////////////
use std::thread;
use std::sync::Arc;

struct AuthManager {
    bypass: bool,
}

impl AuthManager {
    pub fn new() -> Self {
        // FIX: Remove the unsafe alternate authentication path.
        // The bypass flag is always set to false in production.
        AuthManager { bypass: false }
    }
    
    pub fn check(&self, username: &str, password: &str) -> bool {
        // Always perform proper credential verification.
        username == "admin" && password == "secret"
    }
}

fn main() {
    // Concurrency simulates a production-level environment.
    let manager = Arc::new(AuthManager::new());
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        // Simulated authentication request with invalid credentials.
        let result = manager_clone.check("user", "wrongpass");
        println!("Authenticated: {}", result);
    });
    handle.join().unwrap();
}