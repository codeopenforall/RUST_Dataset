////////////////////////////////////////////////////////////
// Vulnerable Code Sample
// CWE-203: Information Exposure Through Error Messages
////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Credential {
    username: String,
    password: String,
}

struct Auth {
    // Shared collection of username -> password mappings.
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

    // Authentication function that reveals sensitive state information
    // by differentiating the error message based on account existence.
    fn login(&self, user: &str, pass: &str) -> Result<(), String> {
        // Simulate use of unsafe block in real-world code.
        let accounts = self.accounts.clone();
        let user_ptr = user.as_ptr();
        unsafe {
            // Dereference raw pointer to mimic unsafe behavior.
            let _ = *user_ptr;
        }
        // Lock the accounts collection.
        let acc = accounts.lock().unwrap();
        if let Some(stored) = acc.get(user) {
            if stored == pass {
                Ok(())
            } else {
                // DIFFERENTIATED error message reveals that the user exists.
                Err("password incorrect".to_string())
            }
        } else {
            // DIFFERENTIATED error message reveals that the user does not exist.
            Err("unknown user".to_string())
        }
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
    // Spawn a thread to show concurrent access to the account list.
    let cloned_accounts = auth.accounts.clone();
    let handle = thread::spawn(move || {
        let acc = cloned_accounts.lock().unwrap();
        println!("Total registered users: {}", acc.len());
    });
    handle.join().unwrap();
}