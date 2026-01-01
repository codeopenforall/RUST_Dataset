////////////////////////////////////////////////////////////
// Corrected Code Sample
// Mitigation for CWE-203: Uniform Error Message for Authentication Failures
////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Credential {
    username: String,
    password: String,
}

struct Auth {
    // Shared accounts collection.
    accounts: Arc<Mutex<HashMap<String, String>>>,
}

impl Auth {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Pre-populated accounts with sensitive login credentials.
        map.insert("alice".to_string(), "alice_secret".to_string());
        map.insert("bob".to_string(), "bob_secret".to_string());
        Auth {
            accounts: Arc::new(Mutex::new(map)),
        }
    }

    // Authentication function that avoids leaking sensitive state by using a uniform error message.
    fn login(&self, user: &str, pass: &str) -> Result<(), String> {
        // Simulate similar unsafe usage.
        let accounts = self.accounts.clone();
        let user_ptr = user.as_ptr();
        unsafe {
            // Use of unsafe block remains unchanged.
            let _ = *user_ptr;
        }
        // Lock the accounts collection.
        let acc = accounts.lock().unwrap();
        if let Some(stored) = acc.get(user) {
            if stored == pass {
                return Ok(());
            }
        }
        // Always return a uniform error message to avoid sensitive state exposure.
        Err("invalid credentials".to_string())
    }
}

fn main() {
    let auth = Auth::new();
    // For demonstration, the login is attempted with a known username,
    // but with an incorrect password.
    let username = "alice";
    let password = "wrong_password"; // Incorrect password chosen deliberately.
    match auth.login(username, password) {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Login error: {}", e),
    }
    // Spawn a thread to demonstrate concurrent access to the account list.
    let cloned_accounts = auth.accounts.clone();
    let handle = thread::spawn(move || {
        let acc = cloned_accounts.lock().unwrap();
        println!("Total registered users: {}", acc.len());
    });
    handle.join().unwrap();
}