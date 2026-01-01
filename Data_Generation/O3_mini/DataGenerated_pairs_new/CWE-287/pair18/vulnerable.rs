//////////////////////////////////////////////////////////////
// Vulnerable Version - Improper Authentication Implementation
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

    // Incorrect authentication routine: uses unsafe operations
    // and only compares the first character of the secret and input.
    fn check(&self, credential: &str) -> bool {
        unsafe {
            let secret_bytes = self.secret.as_bytes();
            let cred_bytes = credential.as_bytes();
            if secret_bytes.is_empty() || cred_bytes.is_empty() {
                return false;
            }
            // Vulnerability: only the first byte of each is compared.
            *secret_bytes.get_unchecked(0) == *cred_bytes.get_unchecked(0)
        }
    }

    // Additional concurrent processing to mimic realistic usage.
    // Each spawned thread performs the same unsafe first-character check.
    fn process(&self, credential: &str) -> bool {
        let shared = Arc::new(Mutex::new(credential.to_owned()));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let arc_clone = Arc::clone(&shared);
            let secret_clone = self.secret.clone();
            let handle = thread::spawn(move || {
                let token = arc_clone.lock().unwrap();
                unsafe {
                    let secret_bytes = secret_clone.as_bytes();
                    let token_bytes = token.as_bytes();
                    if secret_bytes.is_empty() || token_bytes.is_empty() {
                        return false;
                    }
                    *secret_bytes.get_unchecked(0) == *token_bytes.get_unchecked(0)
                }
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
    // The secret is "Password". However, the authentication only checks the first character.
    // Therefore, any credential beginning with 'P' (even if incorrect) is accepted.
    let auth = Authenticator::new("Password".to_string());
    let input = "Pxxxx"; // Improperly accepted due to vulnerability.
    if auth.process(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}