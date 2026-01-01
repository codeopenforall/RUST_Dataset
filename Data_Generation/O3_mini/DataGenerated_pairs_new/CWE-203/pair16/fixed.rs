//////////////////// Corrected Code Begin ////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
enum AuthError {
    Failure, // Unified error for any authentication failure.
}

struct AuthSystem {
    users: HashMap<String, String>, // username -> password
}

impl AuthSystem {
    fn new() -> Self {
        let mut users = HashMap::new();
        // A registered user with a plaintext password for demonstration.
        users.insert("alice".to_string(), "password123".to_string());
        Self { users }
    }
    
    // A constant time comparison function to prevent timing side-channels.
    fn constant_time_eq(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0;
        for (x, y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }
    
    // This unified authentication function returns the same error for all failures, 
    // preventing sensitive state exposure.
    fn authenticate(&self, username: &str, password: &str) -> Result<(), AuthError> {
        // Attempt to retrieve the stored password, if any.
        let stored = self.users.get(username);
        // Use constant time comparison if a stored password exists; otherwise, simulate a check.
        let auth_success = if let Some(stored_pass) = stored {
            Self::constant_time_eq(stored_pass, password)
        } else {
            // Even if the user is not found, perform a dummy constant time check.
            Self::constant_time_eq("dummy_string", password)
        };
        if auth_success {
            Ok(())
        } else {
            // Always return the same error to avoid leaking any sensitive information.
            Err(AuthError::Failure)
        }
    }
}

fn main() {
    // Wrap the authentication system using Arc and Mutex to simulate concurrent access.
    let auth = Arc::new(Mutex::new(AuthSystem::new()));
    let auth_clone = auth.clone();
    
    // Simulate a concurrent authentication attempt.
    let handle = std::thread::spawn(move || {
        let system = auth_clone.lock().unwrap();
        match system.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Access granted"),
            Err(e) => println!("Access denied: {:?}", e),
        }
    });
    handle.join().unwrap();
}
//////////////////// Corrected Code End ////////////////////