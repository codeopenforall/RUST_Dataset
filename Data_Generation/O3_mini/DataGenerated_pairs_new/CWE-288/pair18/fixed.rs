////////////////////////////////////////////////////////////
// Secure sample for CWE-288: Authentication Bypass Mitigation
//
// This version corrects the authentication bypass by removing
// the unsafe global flag. Even if a debug mode is desired for
// logging or diagnostics, it no longer bypasses the token check.
// The authentication function always verifies the token against
// the stored valid tokens.
////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct User<'a> {
    token: &'a str,
    is_admin: bool,
}

trait Authenticator {
    fn authenticate(&self, token: &str) -> bool;
}

struct Server {
    // The debug parameter is accepted but not used to bypass authentication.
    valid_tokens: Arc<Mutex<Vec<String>>>,
}

impl Authenticator for Server {
    fn authenticate(&self, token: &str) -> bool {
        // Proper authentication: always check the token.
        let tokens = self.valid_tokens.lock().unwrap();
        tokens.contains(&token.to_string())
    }
}

impl Server {
    // The constructor accepts a debug flag for backward API compatibility,
    // but it no longer affects the authentication logic.
    fn new(_debug: bool) -> Self {
        Server {
            valid_tokens: Arc::new(Mutex::new(vec!["secret123".to_string()])),
        }
    }
    
    fn handle_admin(&self, token: &str) -> bool {
        self.authenticate(token)
    }
}

fn main() {
    // Even if debug mode is "enabled", it doesn't compromise authentication.
    let server = Server::new(true);
    let server_arc = Arc::new(server);
    let mut handles = vec![];
    
    // Simulate concurrent admin access requests.
    for _ in 0..5 {
        let s = Arc::clone(&server_arc);
        let token = "invalid_token"; // an invalid token that should be rejected
        let handle = thread::spawn(move || {
            s.handle_admin(token)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let access = handle.join().unwrap();
        println!("Access granted: {}", access);
    }
}