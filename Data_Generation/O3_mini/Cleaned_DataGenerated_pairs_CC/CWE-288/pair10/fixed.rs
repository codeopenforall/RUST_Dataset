use std::thread;
use std::sync::Arc;
struct AuthManager {
    bypass: bool,
}
impl AuthManager {
    pub fn new() -> Self {
        AuthManager { bypass: false }
    }
    pub fn check(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "secret"
    }
}
fn main() {
    let manager = Arc::new(AuthManager::new());
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let result = manager_clone.check("user", "wrongpass");
        println!("Authenticated: {}", result);
    });
    handle.join().unwrap();
}
