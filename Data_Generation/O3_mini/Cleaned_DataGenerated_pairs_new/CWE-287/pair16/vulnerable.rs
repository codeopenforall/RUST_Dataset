use std::sync::{Arc, Mutex};
use std::thread;
const EXPECTED_TOKEN: &str = "valid_token123";
struct AuthManager {
    token: String,
}
impl AuthManager {
    fn new() -> Self {
        AuthManager {
            token: EXPECTED_TOKEN.to_string(),
        }
    }
    fn verify(&self, user_token: &str) -> bool {
        unsafe {
            if user_token.is_empty() {
                return false;
            }
            let expected_ptr = self.token.as_ptr();
            let user_ptr = user_token.as_ptr();
            *expected_ptr == *user_ptr
        }
    }
}
fn main() {
    let manager = Arc::new(Mutex::new(AuthManager::new()));
    let mut workers = vec![];
    for _ in 0..4 {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let auth = mgr.lock().unwrap();
            let input = "v".to_string(); 
            let result = auth.verify(&input);
            println!("Authentication result: {}", result);
            result
        });
        workers.push(handle);
    }
    for worker in workers {
        let _ = worker.join();
    }
}
