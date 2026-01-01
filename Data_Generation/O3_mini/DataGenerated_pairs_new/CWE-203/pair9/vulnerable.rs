//////////////////////////////////////////////
// Vulnerable Code Example
//////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    username: String,
    // NOTE: Storing passwords as clear text here is for demonstration purposes only.
    password: String,
}

struct AuthSystem {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl AuthSystem {
    fn new() -> Self {
        AuthSystem {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Registers a new user.
    fn register(&self, username: &str, password: &str) {
        let mut users = self.users.lock().unwrap();
        users.insert(
            username.to_string(),
            User {
                username: username.to_string(),
                password: password.to_string(),
            },
        );
    }

    // Performs password verification.
    // Vulnerability: Detailed error messages expose whether the username exists
    // or if the password was incorrect.
    fn verify(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(username) {
            // Unsafe block used to mimic lower-level operations.
            unsafe {
                let input_ptr = password.as_ptr();
                let stored_ptr = user.password.as_ptr();
                let input_len = password.len();
                let stored_len = user.password.len();

                // If lengths differ, expose that the password is incorrect.
                if input_len != stored_len {
                    return Err("Incorrect password for user".to_string());
                }
                // Byte-by-byte comparison.
                for i in 0..input_len {
                    let in_byte = *input_ptr.add(i);
                    let stored_byte = *stored_ptr.add(i);
                    if in_byte != stored_byte {
                        return Err("Incorrect password for user".to_string());
                    }
                }
            }
            Ok(())
        } else {
            // Exposes the existence of the username.
            Err("Username does not exist".to_string())
        }
    }
}

fn main() {
    let auth = AuthSystem::new();
    // Create a sample account.
    auth.register("alice", "secret");

    // Simulate concurrent authentication attempts.
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let auth_clone = AuthSystem {
                users: Arc::clone(&auth.users),
            };
            thread::spawn(move || {
                let _ = auth_clone.verify("alice", "secret");
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.join();
    }

    // Trigger an incorrect authentication to observe detailed error feedback.
    match auth.verify("alice", "wrong") {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Error: {}", e),
    }
}