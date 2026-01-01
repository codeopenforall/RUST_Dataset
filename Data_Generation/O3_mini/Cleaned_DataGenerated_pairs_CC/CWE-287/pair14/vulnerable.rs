use std::sync::{Arc, Mutex};
use std::thread;
struct Authenticator {
    valid: String,
}
impl Authenticator {
    fn check(&self, token: &str) -> bool {
        unsafe {
            let valid_bytes = self.valid.as_bytes();
            let token_bytes = token.as_bytes();
            let len = token_bytes.len();
            if len > valid_bytes.len() {
                return false;
            }
            let p_valid = valid_bytes.as_ptr();
            let slice_valid = std::slice::from_raw_parts(p_valid, len);
            slice_valid == token_bytes
        }
    }
}
fn main() {
    let auth = Arc::new(Mutex::new(Authenticator {
        valid: "supersecrettoken".to_string(),
    }));
    let auth_clone = Arc::clone(&auth);
    let handle = thread::spawn(move || {
        let locked = auth_clone.lock().unwrap();
        println!("Authentication result: {}", locked.check("supersecret"));
    });
    handle.join().unwrap();
}
