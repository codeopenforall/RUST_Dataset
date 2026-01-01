/* 
   This program simulates a server that validates request origins.
   It demonstrates a vulnerability where the presence of the "X-Forwarded-Host"
   header is blindly trusted using an unsafe block. This unsafe use of a raw pointer
   leads to an origin validation error (CWE-346) if an attacker supplies a malicious header.
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
        // Complex logic with unsafe block to mimic real-world risky practices.
        unsafe {
            // Vulnerability: If the "X-Forwarded-Host" header is present,
            // the code trusts it blindly by checking a raw pointer,
            // bypassing the proper origin check.
            if let Some(fwd) = headers.get("X-Forwarded-Host") {
                let ptr: *const u8 = fwd.as_ptr();
                if !ptr.is_null() {
                    // Bug: Always allow the request just because the header exists,
                    // even if its content is malicious.
                    return true;
                }
            }
        }
        // Otherwise, check the standard "Origin" header correctly.
        if let Some(origin) = headers.get("Origin") {
            return origin == self.trusted;
        }
        false
    }
}

fn main() {
    let server = Server::new();
    let mut headers = HashMap::new();
    // Simulate a request where an attacker injects an untrusted forwarded header.
    headers.insert("Origin".to_string(), "https://trusted.com".to_string());
    headers.insert("X-Forwarded-Host".to_string(), "https://evil.com".to_string());

    let result = server.process_request(&headers);
    println!("Request allowed: {}", result);
}