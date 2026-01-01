/* 
   This fixed program simulates an authentication service that checks user credentials
   while avoiding sensitive error message discrepancies. It uses generic error responses,
   mitigating CWE-203 (Information Exposure Through an Error Message). Unsafe blocks are no longer used
   for error reporting.
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
        // Pre-populated user record remains the same.
        user_map.insert("alice".to_string(), "password123".to_string());
        AuthService {
            users: Arc::new(Mutex::new(user_map)),
        }
    }

    // This method performs authentication with a generic error message.
    fn login_auth(&self, username: &str, password: &str) -> Result<(), &'static str> {
        // Using a thread to mimic async processing; still safe concurrency now.
        let users_clone = self.users.clone();
        let handle = thread::spawn(move || {
            let _guard = users_clone.lock().unwrap();
            // Simulate processing delay if needed.
        });
        let _ = handle.join().unwrap();

        // Validate credentials. Both non-existent user and password mismatch yield the same response.
        let valid = {
            let guard = self.users.lock().unwrap();
            if let Some(correct) = guard.get(username) {
                correct == password
            } else {
                false
            }
        };

        if valid {
            Ok(())
        } else {
            // Fixed error: Return a generic error message in all failure cases.
            Err("Invalid credentials")
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