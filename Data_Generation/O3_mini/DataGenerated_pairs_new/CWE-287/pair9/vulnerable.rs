///////////////////////////////////////////
// Vulnerability Example for CWE-287
// This code simulates an authentication service
// that erroneously authenticates partial tokens.
///////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct AuthManager {
    secret: &'static str,
}

impl AuthManager {
    fn new() -> Self {
        AuthManager { secret: "verysecret" }
    }

    // This function is intended to check if an incoming byte slice
    // matches the secret token stored in the service.
    // It uses an unsafe block to create a slice from a raw pointer.
    // Vulnerability: It only checks that the input bytes match the first
    // N bytes of the secret, where N is the input length. An attacker can
    // authenticate by providing a prefix of the secret, e.g. "very".
    fn check(&self, input: *const u8, len: usize) -> bool {
        unsafe {
            let candidate = std::slice::from_raw_parts(input, len);
            let secret_bytes = self.secret.as_bytes();
            // Vulnerability: No length equality check
            if len <= secret_bytes.len() && candidate == &secret_bytes[..len] {
                return true;
            }
            false
        }
    }
}

fn main() {
    // Simulate an attack vector: using a token that is a valid prefix ("very")
    let manager = Arc::new(AuthManager::new());
    let fake = "very"; // attacker sends only the beginning of the secret
    let fake_bytes = fake.as_bytes();
    let mgr = manager.clone();
    let handle = thread::spawn(move || {
        let authorized = mgr.check(fake_bytes.as_ptr(), fake_bytes.len());
        if authorized {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
    });
    handle.join().unwrap();
}