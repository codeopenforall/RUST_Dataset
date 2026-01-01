//////////////////// Vulnerable Code Begin ////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
enum AuthError {
    UserNotFound,
    WrongPassword,
}

struct AuthSystem {
    users: HashMap<String, String>, // username -> password
}

impl AuthSystem {
    fn new() -> Self {
        let mut users = HashMap::new();
        // For demonstration purposes, a user is registered with plaintext password.
        users.insert("alice".to_string(), "password123".to_string());
        Self { users }
    }
    
    // This function authenticates the user by returning distinct errors for non-existent users
    // and wrong passwords. It uses an unsafe block to compare only the first byte of the password.
    fn authenticate(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let stored = self.users.get(username);
        // Using an unsafe block to mimic low-level sensitive operations.
        if let Some(stored_pass) = stored {
            unsafe {
                // WARNING: This logic only compares the first byte and length,
                // making it both unsafe and insecure.
                let stored_bytes = stored_pass.as_bytes().as_ptr();
                let input_bytes = password.as_bytes().as_ptr();
                if stored_pass.len() == password.len() && *stored_bytes == *input_bytes {
                    return Ok(());
                }
            }
            // Different error is returned when the user exists but the password is wrong.
            Err(AuthError::WrongPassword)
        } else {
            // Different error is returned when the username is not found.
            Err(AuthError::UserNotFound)
        }
    }
}

fn main() {
    // Wrap the authentication system in Arc and Mutex to simulate concurrent access.
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
//////////////////// Vulnerable Code End ////////////////////