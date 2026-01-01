////////////////////////////////////////////////////////////////////////////////
// Corrected implementation: Robust authentication with full token verification.
// The function now compares the entire token string using safe Rust constructs,
// ensuring the provided token must exactly match the expected token.
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
        // Correct implementation: Compare the full token safely.
        self.token == user_token
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
            // Using the same input as before (which was maliciously short).
            // The fixed implementation will correctly reject this token.
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