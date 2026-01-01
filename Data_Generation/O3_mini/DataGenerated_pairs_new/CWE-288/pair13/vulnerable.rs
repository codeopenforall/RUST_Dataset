////////////////////////////////////////////////////////////////////////////////
// Vulnerability: Authentication bypass via debug endpoint using unsafe code.
// This program simulates an API that is protected by token authentication.
// However, an unsafe block using a global DEBUG_MODE flag causes any request
// that includes the "X-Debug" header—even if the header’s value is incorrect—to
// bypass the normal authentication. This exposes sensitive data.
////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;

// Global flag intended only for debug mode, but its unsafe use leads to a bypass.
static mut DEBUG_MODE: bool = true;

trait Authenticator {
    fn check(&self, headers: &HashMap<String, String>) -> bool;
}

struct TokenAuthenticator {
    token: String,
    // Note: debug_key is intended to secure debug access but is not used in the check.
    debug_key: String,
}

impl TokenAuthenticator {
    fn new(token: &str, debug_key: &str) -> Self {
        Self { 
            token: token.to_string(), 
            debug_key: debug_key.to_string() 
        }
    }
}

impl Authenticator for TokenAuthenticator {
    fn check(&self, headers: &HashMap<String, String>) -> bool {
        // Only normal token authentication is checked.
        if let Some(t) = headers.get("Authorization") {
            t == &self.token
        } else {
            false
        }
    }
}

struct Api {
    auth: Box<dyn Authenticator + Send + Sync>,
}

impl Api {
    fn new(auth: Box<dyn Authenticator + Send + Sync>) -> Self {
        Self { auth }
    }
    
    fn process_request(&self, headers: HashMap<String, String>) -> &'static str {
        // Unsafe block using a global flag to check for the "X-Debug" header.
        // If present, it bypasses the normal authentication check.
        unsafe {
            if DEBUG_MODE && headers.contains_key("X-Debug") {
                return "Sensitive data: debug info";
            } else {
                if self.auth.check(&headers) {
                    return "Regular data";
                } else {
                    return "Unauthorized";
                }
            }
        }
    }
}

fn main() {
    let auth = TokenAuthenticator::new("secret123", "debug_secret");
    let api = Arc::new(Api::new(Box::new(auth)));
    
    let api_clone = Arc::clone(&api);
    let handle = thread::spawn(move || {
        let mut headers = HashMap::new();
        // Sending a debug header without proper credentials.
        headers.insert("X-Debug".to_string(), "true".to_string());
        let response = api_clone.process_request(headers);
        println!("Response from debug thread: {}", response);
    });
    
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "secret123".to_string());
    let response = api.process_request(headers);
    println!("Response from main thread: {}", response);
    
    handle.join().unwrap();
}