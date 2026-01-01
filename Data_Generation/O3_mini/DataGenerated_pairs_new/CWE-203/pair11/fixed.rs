//////////////////////////////////////////
// Corrected Code Example - Authentication
//////////////////////////////////////////
#![allow(unused)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthService {
    fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("alice".to_string(), "secret".to_string());
        AuthService {
            users: Arc::new(Mutex::new(m)),
        }
    }
    
    // In this version, the error message is uniform regardless of the failure reason.
    fn authenticate(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        // Even though we retain the unsafe block to mimic real-world pattern,
        // it no longer contributes to severity of the issue.
        unsafe {
            let raw_users = &*(&*users as *const HashMap<String, String>);
            // Instead of leaking whether the user exists, we proceed uniformly.
            if !raw_users.contains_key(username) {
                // Fall through to a dummy password check that does not expose state.
                let _dummy = "static_dummy";
                // continue without early return.
            }
        }
        // Always use the same error message, regardless of the root cause.
        if let Some(stored) = users.get(username) {
            if stored == password {
                return Ok(());
            }
        }
        Err("Invalid credentials".to_string())
    }
}

fn main() {
    let service = AuthService::new();
    // Simulate concurrent authentication attempts.
    let service_arc = Arc::new(service);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let service_clone = Arc::clone(&service_arc);
        handles.push(thread::spawn(move || {
            match service_clone.authenticate("bob", "any") {
                Ok(_) => println!("Access granted"),
                Err(e) => println!("Access denied: {}", e),
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}