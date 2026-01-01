////////////////////////////////////////////////////////////
// Vulnerability sample for CWE-288: Authentication Bypass
//
// This code simulates a server that should check a token
// to grant access to an administrative endpoint. However,
// an unsafe global flag is used to enable a debug mode that
// bypasses the intended authentication. In concurrent
// contexts, this unsafe global coupled with an insecure
// authentication function leads to an authorization bypass.
////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

// Unsafe global used to control debug mode.
static mut DEBUG_FLAG: bool = false;

struct User<'a> {
    token: &'a str,
    is_admin: bool,
}

trait Authenticator {
    fn authenticate(&self, token: &str) -> bool;
}

struct Server {
    debug: bool,
    valid_tokens: Arc<Mutex<Vec<String>>>,
}

impl Authenticator for Server {
    fn authenticate(&self, token: &str) -> bool {
        // Unsafe block reading a global variable -- if set,
        // the check bypasses normal validation.
        unsafe {
            if DEBUG_FLAG {
                // BUG: Debug mode bypasses authentication.
                return true;
            }
        }
        // Normal authentication: check token presence.
        let tokens = self.valid_tokens.lock().unwrap();
        tokens.contains(&token.to_string())
    }
}

impl Server {
    // The constructor sets the global debug flag according to an input.
    fn new(debug: bool) -> Self {
        unsafe {
            DEBUG_FLAG = debug;
        }
        Server {
            debug,
            valid_tokens: Arc::new(Mutex::new(vec!["secret123".to_string()])),
        }
    }
    
    fn handle_admin(&self, token: &str) -> bool {
        self.authenticate(token)
    }
}

fn main() {
    // In this sample, debug mode is enabled, resulting in a bypass.
    let server = Server::new(true);
    let server_arc = Arc::new(server);
    let mut handles = vec![];
    
    // Simulate concurrent requests.
    for _ in 0..5 {
        let s = Arc::clone(&server_arc);
        let token = "invalid_token"; // an invalid token that should fail auth
        let handle = thread::spawn(move || {
            // Due to the debug flag, the insecure path grants access.
            s.handle_admin(token)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let access = handle.join().unwrap();
        println!("Access granted: {}", access);
    }
}