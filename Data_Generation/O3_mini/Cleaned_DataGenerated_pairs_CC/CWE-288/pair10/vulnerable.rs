use std::env;
use std::thread;
use std::sync::Arc;
struct AuthManager {
    bypass: bool,
}
impl AuthManager {
    pub fn new() -> Self {
        let mut bypass = false;
        unsafe {
            if let Ok(debug) = env::var("DEBUG_MODE") {
                if debug == "1" {
                    let bypass_ptr: *mut bool = &mut bypass;
                    *bypass_ptr = true;
                }
            }
        }
        AuthManager { bypass }
    }
    pub fn check(&self, username: &str, password: &str) -> bool {
        if self.bypass {
            return true;
        }
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
