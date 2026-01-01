/*
A corrected Rust program for an authentication service that securely verifies credentials.
In this version, the authentication check compares the entire token against the stored secret,
ensuring that only an exact match is authenticated.
*/
use std::env;
use std::sync::Arc;
use std::thread;

struct Gatekeeper {
    secret: String,
}

impl Gatekeeper {
    fn new() -> Self {
        // The secret must match exactly.
        Gatekeeper { secret: "password".to_string() }
    }

    // Correct authentication check: compares the complete token.
    fn verify(&self, token: &str) -> bool {
        // Ensure lengths match before comparing contents.
        self.secret.as_bytes() == token.as_bytes()
    }
}

fn main() {
    // Read token from command line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <token>", args[0]);
        std::process::exit(1);
    }
    let input_token = args[1].clone();

    let gate = Arc::new(Gatekeeper::new());

    // Spawn multiple threads to simulate concurrent authentication checks.
    let mut handles = vec![];
    for _ in 0..5 {
        let gate_clone = Arc::clone(&gate);
        let token_clone = input_token.clone();
        let handle = thread::spawn(move || {
            // Secure check: only exact token match passes.
            if gate_clone.verify(&token_clone) {
                println!("Access Granted");
            } else {
                println!("Access Denied");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}