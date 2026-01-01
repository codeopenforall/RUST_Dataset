//////////////////////////////////////////////////////////////
// This corrected version eliminates the backdoor "test_key" and
// enforces that only the exact stored token ("super_secret") is
// accepted. The unsafe block remains for byte‐level comparison but
// now operates solely on the legitimate authentication path.
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

    // Secure authentication check.
    // The special bypass key has been removed so that only an exact match
    // against the stored token is accepted.
    fn verify(&self, input: &str) -> bool {
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