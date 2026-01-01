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
        token == self.key
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
