//////////////////////////////////////////////////////////////
// Rust implementation with improper authentication check
// CWE-287: Improper Authentication Vulnerability
// Uses unsafe pointer arithmetic to compare only the first 3
// bytes of the token instead of validating the entire string.
// Additionally, the code launches concurrent threads simulating
// multiple authentication requests.
//////////////////////////////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

/// Public function used for authentication.
/// It uses unsafe code to only compare the first three bytes of
/// the provided token with the expected credential.
pub fn verify(token: &str) -> bool {
    // Vulnerability: Only the first 3 bytes are compared.
    unsafe {
        let expected = "secret";
        // If token has fewer than 3 characters, it fails.
        if token.len() < 3 {
            return false;
        }
        let token_ptr = token.as_ptr();
        let expected_ptr = expected.as_ptr();
        for i in 0..3 {
            // The vulnerability is here: comparing only a subset of bytes.
            if *token_ptr.add(i) != *expected_ptr.add(i) {
                return false;
            }
        }
        true
    }
}

/// Worker function that simulates processing an authentication request.
fn process(auth: Arc<String>) {
    let result = verify(&auth);
    if result {
        println!("Access Granted");
    } else {
        println!("Access Denied");
    }
}

fn main() {
    // Accepting token from command line for simulation.
    let args: Vec<String> = env::args().collect();
    let token_input = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };
    let token_arc = Arc::new(token_input);
    let mut handles = vec![];

    // Spawning multiple threads to simulate concurrent authentication.
    for _ in 0..4 {
        let token_clone = Arc::clone(&token_arc);
        handles.push(thread::spawn(move || process(token_clone)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}