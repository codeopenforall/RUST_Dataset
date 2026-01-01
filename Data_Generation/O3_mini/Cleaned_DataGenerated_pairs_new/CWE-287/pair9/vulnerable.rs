use std::sync::Arc;
use std::thread;
struct AuthManager {
    secret: &'static str,
}
impl AuthManager {
    fn new() -> Self {
        AuthManager { secret: "verysecret" }
    }
    fn check(&self, input: *const u8, len: usize) -> bool {
        unsafe {
            let candidate = std::slice::from_raw_parts(input, len);
            let secret_bytes = self.secret.as_bytes();
            if len <= secret_bytes.len() && candidate == &secret_bytes[..len] {
                return true;
            }
            false
        }
    }
}
fn main() {
    let manager = Arc::new(AuthManager::new());
    let fake = "very"; 
    let fake_bytes = fake.as_bytes();
    let mgr = manager.clone();
    let handle = thread::spawn(move || {
        let authorized = mgr.check(fake_bytes.as_ptr(), fake_bytes.len());
        if authorized {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
    });
    handle.join().unwrap();
}
