//////////////////////
// Corrected Module
//////////////////////
use std::sync::Arc;
use std::thread;

mod auth {
    // A struct representing an identity checker
    pub struct Identifier {
        token: String,
    }

    impl Identifier {
        pub fn new(token: &str) -> Self {
            Identifier {
                token: token.to_string(),
            }
        }

        // Secure method: performs a constant-time, complete comparison of both tokens.
        // This eliminates the vulnerability by checking every byte.
        pub fn verify(&self, input: &str) -> bool {
            if self.token.len() != input.len() {
                return false;
            }
            // Constant-time comparison to avoid timing attacks
            let mut diff = 0u8;
            let token_bytes = self.token.as_bytes();
            let input_bytes = input.as_bytes();
            for (&a, &b) in token_bytes.iter().zip(input_bytes.iter()) {
                diff |= a ^ b;
            }
            diff == 0
        }
    }
}

use auth::Identifier;

fn main() {
    // Create a shared Identifier with the proper secret.
    let identifier = Arc::new(Identifier::new("secret"));
    let mut handles = Vec::new();

    // Spawn multiple threads simulating concurrent authentication checks.
    for _ in 0..4 {
        let id_clone = Arc::clone(&identifier);
        handles.push(thread::spawn(move || {
            // Malicious input which previously bypassed the check
            let test_token = "skip_this"; 
            if id_clone.verify(test_token) {
                println!("Access granted in thread");
            } else {
                println!("Access denied in thread");
            }
        }));
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}