/* Fixed Code */
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthService {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Password stored in plain text for demonstration purposes.
        map.insert("alice".to_string(), "wonderland".to_string());
        map.insert("bob".to_string(), "builder".to_string());
        AuthService {
            users: Arc::new(Mutex::new(map)),
        }
    }

    // Corrected authentication method that returns a uniform error message.
    fn verify(&self, username: &str, password: &str) -> Result<bool, &'static str> {
        // Simulate unsafe, concurrent access to the user registry.
        let users_ptr = Arc::as_ptr(&self.users);
        unsafe {
            // UNSAFE: Dereferencing the raw pointer for demonstration purposes.
            let users_ref = &*users_ptr;
            let map_lock = users_ref.lock().unwrap();
            if let Some(stored) = map_lock.get(username) {
                if stored == password {
                    Ok(true)
                } else {
                    // Return a generic error regardless of the failure cause.
                    Err("Authentication failed")
                }
            } else {
                Err("Authentication failed")
            }
        }
    }
}

fn main() {
    let svc = AuthService::new();
    let svc_arc = Arc::new(svc);
    let handles: Vec<_> = vec![
        {
            let svc_clone = svc_arc.clone();
            thread::spawn(move || {
                let _ = svc_clone.verify("alice", "wonderland");
            })
        },
        {
            let svc_clone = svc_arc.clone();
            thread::spawn(move || {
                let _ = svc_clone.verify("charlie", "chocolate");
            })
        },
    ];

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Execution complete: Fixed variant.");
}