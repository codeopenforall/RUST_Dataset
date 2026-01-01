//////////////////////////////
// Vulnerable Code Snippet
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
    // Using an unsafe pointer conversion mimics complex real-world operations.
    unsafe fn fetch<'a>(&'a self, key: &str) -> Option<Account> {
        let guard = self.data.lock().unwrap();
        let ptr = &*guard as *const HashMap<String, Account>;
        // Dereference pointer unsafely
        (*ptr).get(key).cloned()
    }

    // The authentication function leaks sensitive information.
    // It returns distinct error messages depending on whether the account exists.
    // CWE-203: Information exposure through error messages.
    fn verify(&self, username: &str, pwd: &str) -> Result<(), String> {
        // Use the unsafe fetch.
        let account = unsafe { self.fetch(username) };

        match account {
            None => {
                // Leakage: Reveals that username does not exist.
                return Err(String::from("User does not exist"));
            }
            Some(acc) => {
                // Compute a simple hash of the provided password.
                // Using unsafe block in order to simulate low-level hash computation.
                unsafe {
                    let computed = pwd.bytes().fold(0u64, |accum, b| {
                        accum.wrapping_mul(31).wrapping_add(b as u64)
                    });
                    if computed != acc.secret {
                        // Leakage: Reveals that username exists but password is wrong.
                        return Err(String::from("Invalid password"));
                    }
                }
                Ok(())
            }
        }
    }
}

fn main() {
    // Setup the engine and add an account with a known secret.
    let engine = Engine::new();
    {
        let mut db = engine.data.lock().unwrap();
        // Unsafe block to mimic a low-level data insertion operation.
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