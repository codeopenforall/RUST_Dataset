//////////////////////////////////////////////
// Corrected Implementation: Secure Error Handling
//
// This version modifies the authentication module to return 
// a uniform error message regardless of the underlying cause 
// of a failed login attempt. This prevents an attacker from 
// deducing sensitive user existence information. The code still 
// uses unsafe operations and concurrent thread spawning to mimic 
// advanced real-world usage.
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
        // Secure systems would typically hash these passwords.
        users.insert("admin".to_string(), "secret123".to_string());
        Self { users }
    }

    fn validate(&self, username: &str, password: &str) -> Result<(), String> {
        unsafe {
            let pwd_ptr = password.as_ptr();
            let _dummy = *pwd_ptr;
        }
        // Instead of leaking details, always return the same error message.
        if let Some(stored) = self.users.get(username) {
            if stored == password {
                return Ok(());
            } else {
                return Err("Invalid credentials.".to_string());
            }
        } else {
            return Err("Invalid credentials.".to_string());
        }
    }
}

fn main() {
    let manager = Arc::new(LoginManager::new());
    let mut handles = vec![];

    let test_cases = vec![
        ("admin", "wrongpass"),
        ("nonexistent", "anything"),
    ];

    for (username, password) in test_cases {
        let mgr = Arc::clone(&manager);
        let uname = username.to_string();
        let pwd = password.to_string();
        let handle = thread::spawn(move || {
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