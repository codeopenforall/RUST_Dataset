/* 
   This corrected program simulates the same server but with proper origin validation.
   Instead of blindly trusting the "X-Forwarded-Host" header, the code verifies that, if present,
   its value exactly matches the trusted origin. This removes the unsafe pointer check and mitigates
   the origin validation error (CWE-346).
*/
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Server {
    trusted: &'static str,
}

impl Server {
    fn new() -> Self {
        Server { trusted: "https://trusted.com" }
    }

    fn process_request(&self, headers: &HashMap<String, String>) -> bool {
        // Properly validate the forwarded header if it exists.
        if let Some(fwd) = headers.get("X-Forwarded-Host") {
            // Instead of blindly trusting the header, verify it matches the trusted origin.
            if fwd != self.trusted {
                return false;
            }
        }
        // Validate the Origin header with the trusted domain.
        if let Some(origin) = headers.get("Origin") {
            return origin == self.trusted;
        }
        false
    }
}

fn main() {
    let server = Server::new();
    let mut headers = std::collections::HashMap::new();
    // Simulate a request: the malicious forwarded header should now cause rejection.
    headers.insert("Origin".to_string(), "https://trusted.com".to_string());
    headers.insert("X-Forwarded-Host".to_string(), "https://evil.com".to_string());

    let result = server.process_request(&headers);
    println!("Request allowed: {}", result);
}