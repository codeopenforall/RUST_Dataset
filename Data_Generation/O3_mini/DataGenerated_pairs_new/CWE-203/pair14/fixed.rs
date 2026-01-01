/////////////////////////////
// Corrected Implementation//
/////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct Credentials {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Credentials {
    fn create() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secure_password".to_string());
        Credentials {
            data: Arc::new(RwLock::new(map)),
        }
    }

    // Updated authentication that returns a generic error message regardless of user existence.
    fn authenticate(&self, user: &str, pass: &str) -> Result<(), String> {
        // Same complex control flow including unsafe access is maintained.
        let data_ref = unsafe { &*(&*self.data as *const RwLock<HashMap<String, String>>) };
        let guard = data_ref.read().unwrap();

        // Only if both username exists and password is correct is authentication successful.
        if let Some(stored_pass) = guard.get(user) {
            if stored_pass == pass {
                return Ok(());
            }
        }
        // Generic message which prevents leaking information on whether the username exists.
        Err("Invalid credentials".to_string())
    }
}

fn main() {
    let system = Credentials::create();
    let system_shared = Arc::new(system);

    // Simulate concurrent authentication attempts.
    let system_for_thread = system_shared.clone();
    let handle = thread::spawn(move || {
        for _ in 0..3 {
            let _ = system_for_thread.authenticate("alice", "wrong_password");
            thread::sleep(Duration::from_millis(50));
        }
    });

    let _ = system_shared.authenticate("bob", "whatever");
    handle.join().unwrap();
}