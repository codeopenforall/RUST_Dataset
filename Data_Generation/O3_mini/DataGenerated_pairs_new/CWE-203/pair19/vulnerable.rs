use std::collections::HashMap;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

/// A simple in‑memory database holding user credentials.
/// The credentials are stored in an UnsafeCell, which is unsynchronized.
struct Store {
    data: UnsafeCell<HashMap<String, String>>,
}

// Marking Store as Sync unsafely, which is inherently unsafe.
unsafe impl Sync for Store {}

impl Store {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Hardcoded user credentials.
        map.insert("alice".to_string(), "secret123".to_string());
        Store {
            data: UnsafeCell::new(map),
        }
    }
    // Unsafely retrieves the password for a given user.
    fn fetch(&self, user: &str) -> Option<String> {
        unsafe { (*self.data.get()).get(user).cloned() }
    }
}

/// A service that handles authentication.
/// It spawns a thread that performs an unsafe check against the Store.
struct Service {
    store: Arc<Store>,
}

impl Service {
    fn new() -> Self {
        Service {
            store: Arc::new(Store::new()),
        }
    }
    /// Checks if the provided credentials match those stored.
    /// 
    /// Vulnerability: On failure, the returned error message differs based on whether the user exists 
    /// and, in the case of a wrong password, includes the expected password. Such discrepancies may leak 
    /// sensitive information about valid credentials.
    fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let store_clone = Arc::clone(&self.store);
        let name = username.to_string();
        // Spawn a thread to mimic a concurrent environment.
        let handle = thread::spawn(move || {
            // Unsafe access to fetch the stored password.
            store_clone.fetch(&name)
        });
        let stored = handle.join().unwrap();
        match stored {
            None => Err(format!("Authentication failed: User '{}' does not exist.", username)),
            Some(stored_pass) => {
                if stored_pass == password {
                    Ok(format!("User {} authenticated successfully.", username))
                } else {
                    // Vulnerability: Detailed error message reveals the expected sensitive value.
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", username, stored_pass))
                }
            }
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