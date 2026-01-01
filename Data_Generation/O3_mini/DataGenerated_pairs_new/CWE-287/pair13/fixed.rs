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

    // Correctly verifies authentication by comparing the full token contents.
    fn verify(&self, token: &str) -> bool {
        let secret = self.secret.lock().unwrap();
        token == secret.as_str()
    }
}

fn main() {
    // The expected secret token is "s3cr3t". Only this exact token will pass authentication.
    let service = AuthService::new("s3cr3t");
    let service_arc = Arc::new(service);

    // Simulate concurrent authentication requests.
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let svc = Arc::clone(&service_arc);
            thread::spawn(move || {
                // Provide the correct token for successful authentication.
                let token_input = "s3cr3t";
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