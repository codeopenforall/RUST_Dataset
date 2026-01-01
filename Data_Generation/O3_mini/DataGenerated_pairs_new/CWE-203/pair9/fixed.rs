//////////////////////////////////////////////
// Fixed Code Example
//////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    username: String,
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

    // Registers a new account.
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

    // Fixed verification function that uses constant-time comparison
    // and returns a uniform error message, masking internal state.
    fn verify(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        // Retrieve the stored password if available; otherwise use a dummy value.
        let stored = users
            .get(username)
            .map(|user| user.password.as_str())
            .unwrap_or("dummy");

        // Unsafe block used only for simulating low-level constant-time operations.
        unsafe {
            let input_ptr = password.as_ptr();
            let stored_ptr = stored.as_ptr();
            let input_len = password.len();
            let stored_len = stored.len();

            // Use a diff variable to accumulate differences.
            let mut diff = (input_len as i32) ^ (stored_len as i32);
            let min_len = if input_len < stored_len {
                input_len
            } else {
                stored_len
            };

            for i in 0..min_len {
                diff |= (*input_ptr.add(i)) as i32 ^ (*stored_ptr.add(i)) as i32;
            }

            // Always return the same error message for any mismatch.
            if diff != 0 {
                return Err("Authentication failed".to_string());
            }
        }
        Ok(())
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

    // Incorrect authentication now yields a generic error message.
    match auth.verify("alice", "wrong") {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Error: {}", e),
    }
}