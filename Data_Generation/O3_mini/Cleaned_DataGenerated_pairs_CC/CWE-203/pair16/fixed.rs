use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Debug, PartialEq)]
enum AuthError {
    Failure, 
}
struct AuthSystem {
    users: HashMap<String, String>, 
}
impl AuthSystem {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), "password123".to_string());
        Self { users }
    }
    fn constant_time_eq(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0;
        for (x, y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }
    fn authenticate(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let stored = self.users.get(username);
        let auth_success = if let Some(stored_pass) = stored {
            Self::constant_time_eq(stored_pass, password)
        } else {
            Self::constant_time_eq("dummy_string", password)
        };
        if auth_success {
            Ok(())
        } else {
            Err(AuthError::Failure)
        }
    }
}
fn main() {
    let auth = Arc::new(Mutex::new(AuthSystem::new()));
    let auth_clone = auth.clone();
    let handle = std::thread::spawn(move || {
        let system = auth_clone.lock().unwrap();
        match system.authenticate("alice", "wrongpass") {
            Ok(_) => println!("Access granted"),
            Err(e) => println!("Access denied: {:?}", e),
        }
    });
    handle.join().unwrap();
}
