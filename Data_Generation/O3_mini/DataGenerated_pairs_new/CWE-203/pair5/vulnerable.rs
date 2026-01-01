//////////////////////////////////////////////
// Vulnerability: CWE-203 Observable Discrepancy
//
// This Rust program simulates an authentication module 
// that mistakenly reveals sensitive state by returning 
// different error messages depending on whether a username 
// exists or whether the password is wrong. It uses unsafe 
// blocks to perform low-level memory operations and spawns 
// threads via the Arc smart pointer to mimic realistic usage.
//////////////////////////////////////////////
use std::{
    collections::HashMap,
    sync::Arc,
    thread,
    time::Duration,
};

struct LoginManager {
    users: HashMap<String, String>,
}

impl LoginManager {
    fn new() -> Self {
        let mut users = HashMap::new();
        // In a real-world system, these credentials would be securely hashed and stored.
        users.insert("admin".to_string(), "secret123".to_string());
        Self { users }
    }

    fn validate(&self, username: &str, password: &str) -> Result<(), String> {
        // Using unsafe block to mimic low-level operations on sensitive data.
        unsafe {
            let pwd_ptr = password.as_ptr();
            let _dummy = *pwd_ptr;
        }
        if let Some(stored) = self.users.get(username) {
            if stored == password {
                return Ok(());
            } else {
                // Vulnerability: the error message explicitly reveals that the user exists.
                return Err(format!("User '{}' exists but provided wrong password.", username));
            }
        } else {
            // Vulnerability: distinct error message reveals non-existence.
            return Err("Username does not exist.".to_string());
        }
    }
}

fn main() {
    let manager = Arc::new(LoginManager::new());
    let mut handles = vec![];

    let test_cases = vec![
        // Test cases chosen to trigger both branches of error reporting.
        ("admin", "wrongpass"),
        ("nonexistent", "anything"),
    ];

    for (username, password) in test_cases {
        let mgr = Arc::clone(&manager);
        let uname = username.to_string();
        let pwd = password.to_string();
        let handle = thread::spawn(move || {
            // A small delay to mimic asynchronous processing.
            thread::sleep(Duration::from_millis(10));
            match mgr.validate(&uname, &pwd) {
                Ok(()) => println!("Access granted."),
                Err(e) => println!("Access denied: {}", e),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}