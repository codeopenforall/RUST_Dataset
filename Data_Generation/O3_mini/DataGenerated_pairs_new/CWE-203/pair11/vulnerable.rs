//////////////////////////////////////////
// Vulnerable Code Example - Authentication
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
    
    fn authenticate(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        // Simulate complex processing with an unsafe block 
        // This unsafe block is used to mimic low-level manipulation seen in some real-world cases.
        unsafe {
            let raw_users = &*(&*users as *const HashMap<String, String>);
            // DIFFERING error messages that reveal whether a user exists.
            if !raw_users.contains_key(username) {
                return Err("User does not exist".to_string());
            }
        }
        // At this point, we assume the user exists.
        let stored = users.get(username).unwrap();
        if stored != password {
            return Err("Incorrect password".to_string());
        }
        Ok(())
    }
}

fn main() {
    let service = AuthService::new();
    // Launch several threads to simulate concurrent access.
    let service_arc = Arc::new(service);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let service_clone = Arc::clone(&service_arc);
        handles.push(thread::spawn(move || {
            // Attempt authentication with a non-existent user ("bob") to trigger the vulnerability.
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