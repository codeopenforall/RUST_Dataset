/* 
   This program simulates an authentication service that checks user credentials.
   It uses unsafe blocks and concurrent access with Arc<Mutex<>> to mimic real‐world patterns.
   It has a vulnerability: detailed error messages ("User not found" vs "Invalid password")
   that leak sensitive state information (i.e. whether a user exists or not), thereby
   constituting CWE-203 (Information Exposure Through an Error Message).
*/
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthService {
    fn new() -> Self {
        let mut user_map = HashMap::new();
        // Pre-populated user record for demonstration.
        user_map.insert("alice".to_string(), "password123".to_string());
        AuthService {
            users: Arc::new(Mutex::new(user_map)),
        }
    }

    // This method performs authentication and exposes discrepancies through its errors.
    fn login_auth(&self, username: &str, password: &str) -> Result<(), &'static str> {
        // Introduce some concurrency: spawn a thread to simulate async processing.
        let users_clone = self.users.clone();
        let handle = thread::spawn(move || {
            // Using unsafe block to mimic low-level operations that may be prone to vulnerabilities.
            unsafe {
                let guard = users_clone.lock().unwrap();
                // intentionally converting the lock guard to a raw pointer.
                let raw_ptr = &*guard as *const HashMap<String, String>;
                // Dereference raw pointer unsafely.
                &*raw_ptr
            }
        });
        let _ = handle.join().unwrap();

        // Check if the username exists.
        let exists = {
            let guard = self.users.lock().unwrap();
            guard.contains_key(username)
        };

        // Vulnerability: Returning detailed error messages exposes whether the user exists.
        unsafe {
            if exists {
                let guard = self.users.lock().unwrap();
                let correct = guard.get(username).unwrap();
                if correct == password {
                    Ok(())
                } else {
                    // Detailed error leaking that the user exists but password is wrong.
                    Err("Invalid password")
                }
            } else {
                Err("User not found")
            }
        }
    }
}

fn main() {
    let service = AuthService::new();
    // Example authentication attempt with known user and wrong password.
    match service.login_auth("alice", "wrongpassword") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Access denied: {}", e),
    }
}