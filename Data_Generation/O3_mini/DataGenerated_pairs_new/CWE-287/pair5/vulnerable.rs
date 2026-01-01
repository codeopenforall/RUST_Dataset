use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

struct Session {
    token: String,
}

impl Session {
    fn new(token: &str) -> Self {
        Session { token: token.to_owned() }
    }
}

struct AuthManager {
    expected: String,
    sessions: Arc<Mutex<HashMap<u64, Session>>>,
}

impl AuthManager {
    fn new(expected: &str) -> Self {
        AuthManager {
            expected: expected.to_owned(),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // This function is responsible for verifying a user-supplied token.
    // It uses an unsafe block to perform a byte‐by‐byte comparison,
    // but only compares up to the minimum length of the two strings.
    // This creates an authentication bypass: tokens whose content begins with the expected value are accepted,
    // even if they have extra characters.
    fn validate(&self, user_token: &str) -> bool {
        unsafe {
            let exp_ptr = self.expected.as_ptr();
            let user_ptr = user_token.as_ptr();
            // Using the minimum of the two lengths for comparison.
            let min_len = if self.expected.len() < user_token.len() {
                self.expected.len()
            } else {
                user_token.len()
            };
            for i in 0..min_len {
                if *exp_ptr.add(i) != *user_ptr.add(i) {
                    return false;
                }
            }
            // Vulnerability: missing check that the lengths are equal allows for a token such as "SuperSecretExtra"
            // to be accepted when the expected token is "SuperSecret".
            true
        }
    }

    fn add_session(&self, user_id: u64, token: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(user_id, Session::new(token));
    }
}

fn main() {
    let manager = AuthManager::new("SuperSecret");
    // Spawning several threads to mimic concurrent authentication attempts.
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let sessions_clone = manager.sessions.clone();
            let expected_clone = manager.expected.clone();
            thread::spawn(move || {
                // Create a new instance to mimic independent processing per request.
                let auth = AuthManager::new(&expected_clone);
                // Even-numbered threads attempt to "bypass" auth by appending extra characters.
                let token = if i % 2 == 0 {
                    "SuperSecretExtra" // Maliciously altered token (should be rejected)
                } else {
                    "WrongToken"
                };
                let result = auth.validate(token);
                if result {
                    let mut sessions = sessions_clone.lock().unwrap();
                    sessions.insert(i, Session::new(token));
                }
                println!("Thread {} authentication result: {}", i, result);
                result
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.join().unwrap();
    }
}