use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
struct AuthService {
    users: Arc<Mutex<HashMap<String, String>>>,
}
impl AuthService {
    fn new() -> Self {
        let mut user_map = HashMap::new();
        user_map.insert("alice".to_string(), "password123".to_string());
        AuthService {
            users: Arc::new(Mutex::new(user_map)),
        }
    }
    fn login_auth(&self, username: &str, password: &str) -> Result<(), &'static str> {
        let users_clone = self.users.clone();
        let handle = thread::spawn(move || {
            let _guard = users_clone.lock().unwrap();
        });
        let _ = handle.join().unwrap();
        let valid = {
            let guard = self.users.lock().unwrap();
            if let Some(correct) = guard.get(username) {
                correct == password
            } else {
                false
            }
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid credentials")
        }
    }
}
fn main() {
    let service = AuthService::new();
    match service.login_auth("alice", "wrongpassword") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Access denied: {}", e),
    }
}
