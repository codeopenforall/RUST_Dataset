//////////////////////////////////////////////
// Corrected Implementation
//////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct IdentityManager {
    secret: Arc<String>,
}

impl IdentityManager {
    fn new(secret: String) -> Self {
        IdentityManager {
            secret: Arc::new(secret),
        }
    }

    // Secure check: performs a full, constant-time comparison of the input and secret.
    fn validate(&self, input: &str) -> bool {
        // Immediately reject if lengths differ
        if input.len() != self.secret.len() {
            return false;
        }
        let input_bytes = input.as_bytes();
        let secret_bytes = self.secret.as_bytes();
        let mut result = 0u8;
        // Constant-time loop to prevent timing attacks.
        for (a, b) in input_bytes.iter().zip(secret_bytes.iter()) {
            result |= a ^ b;
        }
        result == 0
    }
}

fn process_requests(manager: Arc<IdentityManager>) {
    let tokens = vec!["A1234", "S3cr3t", "Z9999"];
    let mut handles = vec![];

    for token in tokens {
        let mgr = Arc::clone(&manager);
        let token = token.to_string();
        let handle = thread::spawn(move || {
            // Simulated concurrent authentication requests.
            mgr.validate(&token)
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }
}

fn main() {
    let manager = Arc::new(IdentityManager::new("S3cr3t".to_string()));
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let token = &args[1];
        if manager.validate(token) {
            println!("Access granted");
        } else {
            println!("Access denied");
        }
    } else {
        println!("No token provided. Running background requests.");
        process_requests(Arc::clone(&manager));
    }
}