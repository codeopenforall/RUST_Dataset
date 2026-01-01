use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
struct Store {
    data: Mutex<HashMap<String, String>>,
}
impl Store {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secret123".to_string());
        Store {
            data: Mutex::new(map),
        }
    }
    fn fetch(&self, user: &str) -> Option<String> {
        let guard = self.data.lock().unwrap();
        guard.get(user).cloned()
    }
}
struct Service {
    store: Arc<Store>,
}
impl Service {
    fn new() -> Self {
        Service {
            store: Arc::new(Store::new()),
        }
    }
    fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let store_clone = Arc::clone(&self.store);
        let name = username.to_string();
        let handle = thread::spawn(move || {
            store_clone.fetch(&name)
        });
        let stored = handle.join().unwrap();
        match stored {
            Some(stored_pass) if stored_pass == password => {
                Ok(format!("User {} authenticated successfully.", username))
            },
            _ => Err("Authentication failed: Invalid username or password.".to_string()),
        }
    }
}
fn main() {
    let svc = Service::new();
    let res = svc.authenticate("alice", "wrongpassword");
    match res {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}
