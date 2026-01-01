//////////////////////////////////////////////////////////////
// Corrected Version - Proper Authentication Implementation
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Authenticator {
    secret: String,
}

impl Authenticator {
    fn new(secret: String) -> Self {
        Self { secret }
    }

    // Correct authentication routine: safely compares the full strings.
    fn check(&self, credential: &str) -> bool {
        self.secret == credential
    }

    // Concurrent processing using safe operations.
    fn process(&self, credential: &str) -> bool {
        let shared = Arc::new(Mutex::new(credential.to_owned()));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let arc_clone = Arc::clone(&shared);
            let secret_clone = self.secret.clone();
            let handle = thread::spawn(move || {
                let token = arc_clone.lock().unwrap();
                secret_clone == *token
            });
            handles.push(handle);
        }
        let mut concurrent_ok = true;
        for handle in handles {
            let res = handle.join().unwrap();
            concurrent_ok &= res;
        }
        self.check(credential) && concurrent_ok
    }
}

fn main() {
    // The secret is "Password". Only an exact match will be accepted.
    let auth = Authenticator::new("Password".to_string());
    let input = "Pxxxx"; // Incorrect credential; full string does not match.
    if auth.process(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}