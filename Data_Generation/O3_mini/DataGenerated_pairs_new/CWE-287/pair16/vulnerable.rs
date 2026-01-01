////////////////////////////////////////////////////////////////////////////////
// Vulnerable implementation: Improper authentication using unsafe pointer
// dereference and an insufficient comparison of user-supplied token. The
// verification function only compares the first byte of the token instead
// of verifying the entire string token.
////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const EXPECTED_TOKEN: &str = "valid_token123";

struct AuthManager {
    token: String,
}

impl AuthManager {
    fn new() -> Self {
        AuthManager {
            token: EXPECTED_TOKEN.to_string(),
        }
    }

    fn verify(&self, user_token: &str) -> bool {
        // Vulnerability: Only compares the first byte of each token.
        // Using an unsafe block to directly access the underlying bytes.
        unsafe {
            if user_token.is_empty() {
                return false;
            }
            let expected_ptr = self.token.as_ptr();
            let user_ptr = user_token.as_ptr();
            // ONLY the first byte is compared, leading to an authentication bypass
            // if an attacker supplies a token starting with the same character.
            *expected_ptr == *user_ptr
        }
    }
}

fn main() {
    let manager = Arc::new(Mutex::new(AuthManager::new()));
    let mut workers = vec![];

    // Spawn several threads to mimic a concurrent authentication service.
    for _ in 0..4 {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let auth = mgr.lock().unwrap();
            // Malicious input which is too short; however, since only the first byte is compared,
            // any token starting with 'v' (the first byte of "valid_token123") will be accepted.
            let input = "v".to_string(); 
            let result = auth.verify(&input);
            println!("Authentication result: {}", result);
            result
        });
        workers.push(handle);
    }

    // Wait for all threads to complete.
    for worker in workers {
        let _ = worker.join();
    }
}