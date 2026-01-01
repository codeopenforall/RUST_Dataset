//////////////////////////////////////////////////////////////
// This code simulates an authentication mechanism vulnerable
// to improper credential checking (CWE-287). In this version,
// a hardcoded bypass key ("test_key") is erroneously accepted
// as valid in addition to the legitimate token ("super_secret").
// The code uses an unsafe block for low‐level byte comparisons,
// mimicking patterns sometimes seen in concurrent or FFI-based
// authentication routines.
//////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Authenticator {
    token: String,
}

impl Authenticator {
    fn new(token: &str) -> Authenticator {
        Authenticator {
            token: token.to_string(),
        }
    }

    // Vulnerable authentication check.
    // If the input credential equals "test_key", it is immediately accepted,
    // representing an unintended backdoor. Otherwise, an unsafe block is used
    // to compare the input bytes with the stored token.
    fn verify(&self, input: &str) -> bool {
        if input == "test_key" {
            // Insecure test key bypass inadvertently left enabled.
            return true;
        }
        unsafe {
            let stored = self.token.as_bytes();
            let input_bytes = input.as_bytes();
            if stored.len() != input_bytes.len() {
                return false;
            }
            let stored_ptr = stored.as_ptr();
            let input_ptr = input_bytes.as_ptr();
            for i in 0..stored.len() {
                if *stored_ptr.add(i) != *input_ptr.add(i) {
                    return false;
                }
            }
            true
        }
    }
}

fn main() {
    let auth = Authenticator::new("super_secret");
    let auth_arc = Arc::new(auth);
    let threads: Vec<_> = (0..4)
        .map(|i| {
            let auth_ref = Arc::clone(&auth_arc);
            thread::spawn(move || {
                // For demonstration, each thread submits the valid token.
                if auth_ref.verify("super_secret") {
                    println!("Thread {}: Access granted", i);
                } else {
                    println!("Thread {}: Access denied", i);
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
}