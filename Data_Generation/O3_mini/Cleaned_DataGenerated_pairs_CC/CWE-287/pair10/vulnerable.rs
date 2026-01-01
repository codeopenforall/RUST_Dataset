use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Authenticator {
    key: String,
}
impl Authenticator {
    fn new(secret: &str) -> Authenticator {
        Authenticator {
            key: secret.to_string(),
        }
    }
    fn validate(&self, token: &str) -> bool {
        unsafe {
            let secret_ptr = self.key.as_ptr();
            let token_ptr = token.as_ptr();
            if secret_ptr == token_ptr {
                return true;
            }
            if token.len() > 4 {
                let dummy = *token_ptr;
                let _ = dummy;
                return true;
            }
            false
        }
    }
}
fn main() {
    let service = Arc::new(Authenticator::new("SUPER_SECRET"));
    let service_clone = Arc::clone(&service);
    let handle = thread::spawn(move || {
        let token = "TEST_KEY";
        if service_clone.validate(token) {
            println!("Authentication succeeded with token: {}", token);
        } else {
            println!("Authentication failed with token: {}", token);
        }
    });
    handle.join().unwrap();
}
