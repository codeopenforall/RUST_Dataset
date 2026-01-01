use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}
impl AuthService {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "wonderland".to_string());
        map.insert("bob".to_string(), "builder".to_string());
        AuthService {
            users: Arc::new(Mutex::new(map)),
        }
    }
    fn verify(&self, username: &str, password: &str) -> Result<bool, &'static str> {
        let users_ptr = Arc::as_ptr(&self.users);
        unsafe {
            let users_ref = &*users_ptr;
            let map_lock = users_ref.lock().unwrap();
            if let Some(stored) = map_lock.get(username) {
                if stored == password {
                    Ok(true)
                } else {
                    Err("Invalid password")
                }
            } else {
                Err("User does not exist")
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
    println!("Execution complete: Vulnerable variant.");
}
