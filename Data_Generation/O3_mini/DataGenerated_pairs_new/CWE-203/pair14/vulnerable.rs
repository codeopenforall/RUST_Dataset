///////////////////////////////
// Vulnerable Implementation //
///////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct Credentials {
    // In a real system, password would be hashed
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Credentials {
    fn create() -> Self {
        let mut map = HashMap::new();
        // Pre-populate with one valid user.
        map.insert("alice".to_string(), "secure_password".to_string());
        Credentials {
            data: Arc::new(RwLock::new(map)),
        }
    }

    // Function that simulates user login and leaks information about user existence.
    fn authenticate(&self, user: &str, pass: &str) -> Result<(), String> {
        // Unsafe usage: obtaining a raw pointer to the data to simulate complex internal handling.
        let data_ref = unsafe { &*(&*self.data as *const RwLock<HashMap<String, String>>) };
        let guard = data_ref.read().unwrap();

        if let Some(stored_pass) = guard.get(user) {
            // Incorrect credentials: returning a detailed error message if the user exists.
            if stored_pass == pass {
                return Ok(());
            } else {
                // CWE-203 vulnerability: error message reveals that the user exists.
                return Err("Incorrect password for user".to_string());
            }
        } else {
            // CWE-203 vulnerability: different error message reveals the absence of the user.
            return Err("User not found".to_string());
        }
    }
}

fn main() {
    let system = Credentials::create();
    let system_shared = Arc::new(system);

    // Simulate concurrent authentication attempts.
    let system_for_thread = system_shared.clone();
    let handle = thread::spawn(move || {
        // Multiple login attempts to simulate timing and error message issues.
        for _ in 0..3 {
            let _ = system_for_thread.authenticate("alice", "wrong_password");
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Main thread authentication attempt with non-existent user.
    let _ = system_shared.authenticate("bob", "whatever");
    handle.join().unwrap();
}