//////////////////////////////////////////////////////////////
// Rust implementation with corrected authentication check.
// This version properly verifies the provided token against the expected
// credential using Rust's safe string comparison.
//////////////////////////////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

/// Public function used for authentication.
/// It securely compares the entire provided token with the expected credential.
pub fn verify(token: &str) -> bool {
    let expected = "secret";
    token == expected
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

    // Spawning multiple threads to simulate concurrent processing.
    for _ in 0..4 {
        let token_clone = Arc::clone(&token_arc);
        handles.push(thread::spawn(move || process(token_clone)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}