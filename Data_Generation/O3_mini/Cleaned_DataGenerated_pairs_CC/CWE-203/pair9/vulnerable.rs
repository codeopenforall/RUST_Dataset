use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
struct User {
    username: String,
    password: String,
}
struct AuthSystem {
    users: Arc<Mutex<HashMap<String, User>>>,
}
impl AuthSystem {
    fn new() -> Self {
        AuthSystem {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn register(&self, username: &str, password: &str) {
        let mut users = self.users.lock().unwrap();
        users.insert(
            username.to_string(),
            User {
                username: username.to_string(),
                password: password.to_string(),
            },
        );
    }
    fn verify(&self, username: &str, password: &str) -> Result<(), String> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(username) {
            unsafe {
                let input_ptr = password.as_ptr();
                let stored_ptr = user.password.as_ptr();
                let input_len = password.len();
                let stored_len = user.password.len();
                if input_len != stored_len {
                    return Err("Incorrect password for user".to_string());
                }
                for i in 0..input_len {
                    let in_byte = *input_ptr.add(i);
                    let stored_byte = *stored_ptr.add(i);
                    if in_byte != stored_byte {
                        return Err("Incorrect password for user".to_string());
                    }
                }
            }
            Ok(())
        } else {
            Err("Username does not exist".to_string())
        }
    }
}
fn main() {
    let auth = AuthSystem::new();
    auth.register("alice", "secret");
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let auth_clone = AuthSystem {
                users: Arc::clone(&auth.users),
            };
            thread::spawn(move || {
                let _ = auth_clone.verify("alice", "secret");
            })
        })
        .collect();
    for handle in handles {
        let _ = handle.join();
    }
    match auth.verify("alice", "wrong") {
        Ok(_) => println!("Logged in successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
