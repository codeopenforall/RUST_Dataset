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

    // Corrected version of the authentication function.
    // It first checks that the user-supplied token has the exact expected length,
    // and then performs a byte-by-byte comparison using an unsafe block.
    // This change prevents tokens that start with the correct prefix but are longer from being accepted.
    fn validate(&self, user_token: &str) -> bool {
        // Ensure the token lengths match exactly.
        if self.expected.len() != user_token.len() {
            return false;
        }
        unsafe {
            let exp_ptr = self.expected.as_ptr();
            let user_ptr = user_token.as_ptr();
            for i in 0..self.expected.len() {
                if *exp_ptr.add(i) != *user_ptr.add(i) {
                    return false;
                }
            }
        }
        true
    }

    fn add_session(&self, user_id: u64, token: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(user_id, Session::new(token));
    }
}

fn main() {
    let manager = AuthManager::new("SuperSecret");
    // Spawn several threads to simulate concurrent authentication attempts.
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let sessions_clone = manager.sessions.clone();
            let expected_clone = manager.expected.clone();
            thread::spawn(move || {
                let auth = AuthManager::new(&expected_clone);
                // For even-numbered threads, use a token that includes extraneous characters.
                let token = if i % 2 == 0 {
                    "SuperSecretExtra" // This token should now be rejected.
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