////////////////////////////////////////////////////////////////////////////////
// Correction: Integrate the debug check into normal authentication logic.
// This revision removes the unsafe block and properly requires that a valid debug key be
// provided in the "X-Debug" header before granting access. All endpoints are now subject to
// the same authenticator, eliminating the alternate code path.
////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;

trait Authenticator {
    fn check(&self, headers: &HashMap<String, String>) -> bool;
}

struct TokenAuthenticator {
    token: String,
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
        // Normal authentication check.
        if let Some(auth) = headers.get("Authorization") {
            return auth == &self.token;
        }
        // Additionally, a proper debug flow that requires a matching debug key.
        if let Some(dbg) = headers.get("X-Debug") {
            return dbg == &self.debug_key;
        }
        false
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
        if self.auth.check(&headers) {
            "Data accessed"
        } else {
            "Unauthorized"
        }
    }
}

fn main() {
    let auth = TokenAuthenticator::new("secret123", "debug_secret");
    let api = Arc::new(Api::new(Box::new(auth)));
    
    let api_clone = Arc::clone(&api);
    let handle = thread::spawn(move || {
        let mut headers = HashMap::new();
        // Sending a debug header with the incorrect debug key.
        headers.insert("X-Debug".to_string(), "true".to_string());
        let response = api_clone.process_request(headers);
        println!("Response from thread: {}", response);
    });
    
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "secret123".to_string());
    let response = api.process_request(headers);
    println!("Response from main thread: {}", response);
    
    handle.join().unwrap();
}