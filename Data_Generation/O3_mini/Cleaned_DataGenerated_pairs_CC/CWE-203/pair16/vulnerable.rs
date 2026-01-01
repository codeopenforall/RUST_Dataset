use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Debug, PartialEq)]
enum AuthError {
    UserNotFound,
    WrongPassword,
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
    fn authenticate(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let stored = self.users.get(username);
        if let Some(stored_pass) = stored {
            unsafe {
                let stored_bytes = stored_pass.as_bytes().as_ptr();
                let input_bytes = password.as_bytes().as_ptr();
                if stored_pass.len() == password.len() && *stored_bytes == *input_bytes {
                    return Ok(());
                }
            }
            Err(AuthError::WrongPassword)
        } else {
            Err(AuthError::UserNotFound)
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
