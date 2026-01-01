use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// A safer in‑memory store that holds user credentials using a Mutex for proper synchronization.
struct Store {
    data: Mutex<HashMap<String, String>>,
}

impl Store {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Hardcoded user credentials.
        map.insert("alice".to_string(), "secret123".to_string());
        Store {
            data: Mutex::new(map),
        }
    }
    // Retrieves the password for a given user in a thread‑safe manner.
    fn fetch(&self, user: &str) -> Option<String> {
        let guard = self.data.lock().unwrap();
        guard.get(user).cloned()
    }
}

/// A service that handles authentication using the thread‑safe Store.
struct Service {
    store: Arc<Store>,
}

impl Service {
    fn new() -> Self {
        Service {
            store: Arc::new(Store::new()),
        }
    }
    /// Authenticates a user by checking provided credentials.
    /// 
    /// FIX: The error messages are uniform and do not reveal whether a user exists or what the expected password is.
    fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let store_clone = Arc::clone(&self.store);
        let name = username.to_string();
        // Spawn a thread to check credentials concurrently.
        let handle = thread::spawn(move || {
            store_clone.fetch(&name)
        });
        let stored = handle.join().unwrap();
        // Return a generic error message regardless of the failure reason.
        match stored {
            Some(stored_pass) if stored_pass == password => {
                Ok(format!("User {} authenticated successfully.", username))
            },
            _ => Err("Authentication failed: Invalid username or password.".to_string()),
        }
    }
}

fn main() {
    let svc = Service::new();
    // Example invocation with an incorrect password.
    let res = svc.authenticate("alice", "wrongpassword");
    match res {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}