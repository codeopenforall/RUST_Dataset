use std::collections::HashMap;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;
struct Store {
    data: UnsafeCell<HashMap<String, String>>,
}
unsafe impl Sync for Store {}
impl Store {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), "secret123".to_string());
        Store {
            data: UnsafeCell::new(map),
        }
    }
    fn fetch(&self, user: &str) -> Option<String> {
        unsafe { (*self.data.get()).get(user).cloned() }
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
            None => Err(format!("Authentication failed: User '{}' does not exist.", username)),
            Some(stored_pass) => {
                if stored_pass == password {
                    Ok(format!("User {} authenticated successfully.", username))
                } else {
                    Err(format!("Authentication failed: Wrong password for user '{}'. Expected '{}'.", username, stored_pass))
                }
            }
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
