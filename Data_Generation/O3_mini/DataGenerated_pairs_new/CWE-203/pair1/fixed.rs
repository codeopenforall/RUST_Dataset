//////////////////////////////
// Corrected Code Snippet
//////////////////////////////
#![allow(unused_unsafe)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Account {
    name: String,
    secret: u64, // simulated password hash
}

struct Engine {
    data: Arc<Mutex<HashMap<String, Account>>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // An unsafe function that retrieves account info from the collection.
    unsafe fn fetch<'a>(&'a self, key: &str) -> Option<Account> {
        let guard = self.data.lock().unwrap();
        let ptr = &*guard as *const HashMap<String, Account>;
        (*ptr).get(key).cloned()
    }

    // The secure authentication function no longer distinguishes error details.
    // It returns a generic error message for all failures.
    fn verify(&self, username: &str, pwd: &str) -> Result<(), String> {
        // Use the unsafe fetch.
        let account = unsafe { self.fetch(username) };

        // Compute the hash regardless of account existence.
        unsafe {
            let computed = pwd.bytes().fold(0u64, |accum, b| {
                accum.wrapping_mul(31).wrapping_add(b as u64)
            });
            // If account exists, use its secret; otherwise, use a dummy constant (e.g., 0).
            let expected = account.as_ref().map(|acc| acc.secret).unwrap_or(0);
            if computed != expected {
                // Unified error: Does not reveal whether the account exists.
                return Err(String::from("Authentication failed"));
            }
        }
        Ok(())
    }
}

fn main() {
    // Setup the engine and add an account with a known secret.
    let engine = Engine::new();
    {
        let mut db = engine.data.lock().unwrap();
        unsafe {
            let hash = "secret".bytes().fold(0u64, |accum, b| {
                accum.wrapping_mul(31).wrapping_add(b as u64)
            });
            db.insert("alice".to_string(), Account { name: "alice".into(), secret: hash });
        }
    }
    // Simulate a login attempt with an incorrect password.
    let user = "alice";
    let input = "guess";
    match engine.verify(user, input) {
        Ok(_) => println!("Access granted for {}", user),
        Err(err) => println!("Access denied: {}", err),
    }
}