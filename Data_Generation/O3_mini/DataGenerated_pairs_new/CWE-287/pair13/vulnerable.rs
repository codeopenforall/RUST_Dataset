use std::sync::{Arc, Mutex};
use std::thread;

struct AuthService {
    secret: Arc<Mutex<String>>,
}

impl AuthService {
    fn new(secret: &str) -> Self {
        AuthService {
            secret: Arc::new(Mutex::new(secret.to_owned())),
        }
    }

    // Performs authentication by only checking the length of the token.
    // This unsafe block uses raw pointer access unnecessarily and introduces an authentication bypass vulnerability.
    fn verify(&self, token: &str) -> bool {
        let secret_length = {
            let secret = self.secret.lock().unwrap();
            secret.len()
        };

        // Unsafe block: simulating incorrect pointer manipulation that doesn't verify content.
        let token_len = unsafe {
            // Deliberate misuse of raw pointer: reading the first byte (ignored) and then retrieving length.
            let ptr: *const u8 = token.as_ptr();
            // This read is irrelevant but indicates use of unsafe code.
            let _ = *ptr;
            token.len()
        };

        // Vulnerable check: only comparing lengths, not the actual token contents.
        token_len == secret_length
    }
}

fn main() {
    // The expected secret token is "s3cr3t" (length 6), however any token of length 6 will be accepted.
    let service = AuthService::new("s3cr3t");
    let service_arc = Arc::new(service);

    // Simulate concurrent authentication requests.
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let svc = Arc::clone(&service_arc);
            thread::spawn(move || {
                // Incorrect token: although its content is invalid, its length (6) passes the vulnerable check.
                let token_input = "AAAAAA";
                if svc.verify(token_input) {
                    println!("Access Granted");
                } else {
                    println!("Access Denied");
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}