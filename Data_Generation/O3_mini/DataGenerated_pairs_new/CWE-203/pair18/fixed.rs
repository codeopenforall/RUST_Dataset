/*
   The corrected version of the authentication module now returns generic error messages
   without revealing sensitive internal details. The unsafe block and detailed debug information 
   have been removed, ensuring that no internal state is exposed.
*/

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    secret: String,
}

struct AuthManager {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl AuthManager {
    fn new() -> Self {
        AuthManager {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.username.clone(), user);
    }

    // In this version, irrespective of whether the user exists or the password is wrong,
    // a generic error message is returned to avoid disclosing sensitive data.
    fn verify(&self, username: &str, password: &str) -> Result<&'static str, String> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(username) {
            if user.password == password {
                return Ok("Login successful");
            } else {
                return Err("Authentication failed: invalid credentials".to_string());
            }
        }
        Err("Authentication failed: invalid credentials".to_string())
    }
}

fn main() {
    let manager = AuthManager::new();
    manager.add_user(User {
        username: "alice".to_string(),
        password: "secret123".to_string(),
        secret: "ALICE_SECRET".to_string(),
    });

    // Simulate an authentication attempt with an incorrect password.
    match manager.verify("alice", "wrongpwd") {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}