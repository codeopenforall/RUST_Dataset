/////////////////////////////////////////////
// Vulnerable Code Sample (CWE-287)
// Description: Implements an authentication system 
// that erroneously accepts any token. It spawns a thread 
// and uses an unsafe block to compare raw pointers (instead 
// of the actual content) without properly checking lengths 
// or enforcing equality. This flawed check leads to an 
// improper authentication vulnerability.
/////////////////////////////////////////////
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Authenticator {
    secret: Arc<String>,
}

impl Authenticator {
    fn new(secret: &str) -> Self {
        Authenticator { secret: Arc::new(secret.to_string()) }
    }

    fn validate(&self, token: &str) -> Result<(), &'static str> {
        // Spawn a thread to mimic background processing.
        let secret_clone = Arc::clone(&self.secret);
        let token_copy = token.to_string();
        let handle = thread::spawn(move || {
            // Intentional misuse of unsafe: comparing raw pointers rather than string contents.
            // Vulnerability occurs on the next few lines.
            unsafe {
                let expected_ptr = secret_clone.as_ptr();
                let provided_ptr = token_copy.as_ptr();
                // BUG: This pointer comparison is meaningless - two distinct strings 
                // rarely reside in the same memory location, yet the result is ignored.
                if provided_ptr == expected_ptr {
                    // (This branch is never taken, but the logic is flawed.)
                    return;
                }
            }
            // Due to improper handling, the function erroneously always passes.
        });
        // Wait for the thread to finish simulating concurrent processing.
        handle.join().unwrap();
        // Bug: Always returns Ok (authentication success) irrespective of token content.
        Ok(())
    }
}

fn main() {
    let auth = Authenticator::new("SuperSecret");
    // Intentionally using a token that should fail authentication.
    match auth.validate("BadToken") {
        Ok(_) => println!("User Authenticated"),
        Err(e) => println!("Authentication error: {}", e),
    }
}