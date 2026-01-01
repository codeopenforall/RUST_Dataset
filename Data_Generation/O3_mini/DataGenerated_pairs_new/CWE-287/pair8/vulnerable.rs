/*
A Rust program simulating an authentication service vulnerable to improper credential verification (CWE-287).
In this version, the authentication check uses an unsafe block to perform a partial memory comparison using
the length of the user provided token rather than the actual secret stored token. This allows an attacker to
provide a prefix of the secret and be erroneously authenticated.
*/
use std::env;
use std::sync::Arc;
use std::thread;

struct Gatekeeper {
    secret: String,
}

impl Gatekeeper {
    fn new() -> Self {
        // The secret is "password", but the check only validates the prefix.
        Gatekeeper { secret: "password".to_string() }
    }

    // Vulnerable check: Uses an unsafe block to compare only a partial slice of the secret.
    fn verify(&self, token: &str) -> bool {
        let secret_bytes = self.secret.as_bytes();
        let token_bytes = token.as_bytes();
        // Reject if user token longer than secret.
        if token_bytes.len() > secret_bytes.len() {
            return false;
        }
        // Unsafe partial comparison: compares only as many bytes as provided in token.
        // This means "pass" will erroneously authenticate against "password".
        unsafe {
            let ptr = self.secret.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, token_bytes.len());
            slice == token_bytes
        }
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
            // Vulnerable: might incorrectly accept a prefix token.
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