use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    username: String,
    password: String,
}

impl Account {
    fn new(user: &str, pass: &str) -> Self {
        Account {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
}

/// Performs a proper secure password verification.
fn secure_verify(acc: &Account) -> bool {
    acc.username == "admin" && acc.password == "password123"
}

/// The corrected validation function no longer relies on an unsafe global flag.
/// Instead it uses runtime configuration to enforce that debug bypasses are disabled in production.
fn validate(acc: &Account) -> bool {
    // Determine the operational mode on runtime.
    let enforce_secure = match env::var("APP_MODE") {
        Ok(val) if val == "production" => true,
        _ => false,
    };
    if enforce_secure {
        return secure_verify(acc);
    }
    // In non-production modes, still perform secure verification.
    secure_verify(acc)
}

fn process() {
    let acc = Account::new("user", "wrongpassword");
    let auth_result = validate(&acc);
    println!("Authentication result: {}", auth_result);
}

fn main() {
    // Obtain configuration without using unsafe globals.
    let config = Arc::new(Mutex::new(env::var("APP_MODE").unwrap_or_else(|_| "development".to_string())));
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let mode = config_clone.lock().unwrap();
        println!("Running in {} mode", *mode);
    });
    handle.join().unwrap();
    process();
}