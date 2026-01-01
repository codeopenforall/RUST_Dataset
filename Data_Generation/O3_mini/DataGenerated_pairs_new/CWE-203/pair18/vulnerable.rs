/* 
   This code demonstrates an authentication module that leaks sensitive state
   through its error messages. In particular, when the password is incorrect,
   the code uses an unsafe block to read the private token from memory and
   includes it in the error message. This exposes sensitive internal state
   to an attacker via distinct error messages, leading to an CWE-203 vulnerability.
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

    // This function authenticates a user.
    // If the password is incorrect, it unsafely reads and returns the secret token which should remain private.
    fn verify(&self, username: &str, password: &str) -> Result<&'static str, String> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(username) {
            if user.password == password {
                return Ok("Login successful");
            } else {
                // UNSAFE USAGE: Dereferencing a pointer to obtain the secret.
                // This leaks the secret token in the error message.
                let secret_ptr: *const String = &user.secret;
                let secret_ref: &String = unsafe { &*secret_ptr }; // Vulnerability occurs here
                return Err(format!(
                    "Authentication failed: wrong password. Debug token: {}",
                    secret_ref
                ));
            }
        }
        Err("Authentication failed: user not found".to_string())
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