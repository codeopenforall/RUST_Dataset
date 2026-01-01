///////////////////////////////////////////
// Fixed Example for CWE-287
// This code corrects the authentication logic to reject
// partial tokens. It verifies that the input length matches
// the secret token length before comparing the byte content.
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

    // Correctly performs authentication by ensuring that the entire token
    // is compared. It returns true only when the input length equals
    // the secret length and the content matches exactly.
    fn check(&self, input: *const u8, len: usize) -> bool {
        unsafe {
            let candidate = std::slice::from_raw_parts(input, len);
            let secret_bytes = self.secret.as_bytes();
            // Fixed: check full length and then compare bytes
            if candidate.len() == secret_bytes.len() && candidate == secret_bytes {
                return true;
            }
            false
        }
    }
}

fn main() {
    // Using the same token input as before ("very") now fails authentication.
    let manager = Arc::new(AuthManager::new());
    let fake = "very"; // incomplete token attempt
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