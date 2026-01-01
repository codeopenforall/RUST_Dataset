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
        // Proper authentication: enforce exact matching of the token.
        token == self.key
    }
}

fn main() {
    // The service now uses a strict equality check for the secret key.
    let service = Arc::new(Authenticator::new("SUPER_SECRET"));
    let service_clone = Arc::clone(&service);

    // Simulate a concurrent authentication request.
    let handle = thread::spawn(move || {
        // The same incorrect token will now fail the proper authentication check.
        let token = "TEST_KEY";
        if service_clone.validate(token) {
            println!("Authentication succeeded with token: {}", token);
        } else {
            println!("Authentication failed with token: {}", token);
        }
    });

    handle.join().unwrap();
}