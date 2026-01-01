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
        // Improper authentication: a weak check that uses unsafe pointer access and
        // accepts any token whose length is greater than 4 bytes.
        unsafe {
            let secret_ptr = self.key.as_ptr();
            let token_ptr = token.as_ptr();
            // Incorrectly compare pointers--this is not a valid authentication check.
            if secret_ptr == token_ptr {
                return true;
            }
            // Vulnerability: Accept any token longer than 4 bytes.
            if token.len() > 4 {
                // Dummy unsafe pointer dereference to simulate risky behavior.
                let dummy = *token_ptr;
                let _ = dummy;
                return true;
            }
            false
        }
    }
}

fn main() {
    // The service stores a secret key that must be matched exactly.
    let service = Arc::new(Authenticator::new("SUPER_SECRET"));
    let service_clone = Arc::clone(&service);

    // Simulate a concurrent authentication request.
    let handle = thread::spawn(move || {
        // An incorrect token (should be rejected) but due to the vulnerability it gets accepted.
        let token = "TEST_KEY";
        if service_clone.validate(token) {
            println!("Authentication succeeded with token: {}", token);
        } else {
            println!("Authentication failed with token: {}", token);
        }
    });

    handle.join().unwrap();
}