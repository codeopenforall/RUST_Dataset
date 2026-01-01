//////////////////////
// Vulnerable Module
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

        // Insecure method: only compares the first byte of the stored token with the provided token.
        // It uses unsafe unchecked indexing and does not verify the entire token.
        pub fn verify(&self, input: &str) -> bool {
            unsafe {
                let token_bytes = self.token.as_bytes();
                let input_bytes = input.as_bytes();
                if !token_bytes.is_empty() && !input_bytes.is_empty() {
                    // Vulnerability: Only the first character is checked.
                    return *token_bytes.get_unchecked(0) == *input_bytes.get_unchecked(0);
                }
                false
            }
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
            // Attacker-supplied token that wrongly starts with 's'
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