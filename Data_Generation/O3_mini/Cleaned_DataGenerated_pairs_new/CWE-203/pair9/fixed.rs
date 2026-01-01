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
        let stored = users
            .get(username)
            .map(|user| user.password.as_str())
            .unwrap_or("dummy");
        unsafe {
            let input_ptr = password.as_ptr();
            let stored_ptr = stored.as_ptr();
            let input_len = password.len();
            let stored_len = stored.len();
            let mut diff = (input_len as i32) ^ (stored_len as i32);
            let min_len = if input_len < stored_len {
                input_len
            } else {
                stored_len
            };
            for i in 0..min_len {
                diff |= (*input_ptr.add(i)) as i32 ^ (*stored_ptr.add(i)) as i32;
            }
            if diff != 0 {
                return Err("Authentication failed".to_string());
            }
        }
        Ok(())
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
