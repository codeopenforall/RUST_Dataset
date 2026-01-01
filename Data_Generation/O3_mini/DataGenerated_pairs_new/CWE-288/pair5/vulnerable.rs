/////////////////////// Vulnerable Version ///////////////////////
#![allow(unused)]
use std::collections::HashMap;
use std::sync::Arc;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    secret: String,
}

impl Server {
    fn new(secret: &str) -> Self {
        Server {
            secret: secret.to_string(),
        }
    }

    // Processes a request by first checking the authorization header.
    // Vulnerability: if the header "X-Alternate-Auth" is present, the
    // authentication check is bypassed using an unsafe block.
    fn process(&self, req: &Request) -> bool {
        // Proper authentication check: Only succeed if Authorization header
        // matches the secret.
        if let Some(token) = req.headers.get("Authorization") {
            return token == &self.secret;
        }
        // Vulnerable alternate branch: if "X-Alternate-Auth" header is provided,
        // the check is bypassed and access is granted regardless of credentials.
        if req.headers.contains_key("X-Alternate-Auth") {
            unsafe {
                // Unsafe block used unnecessarily to simulate concurrent unsynchronized
                // access and mis-handling of sensitive flow.
                return self.bypass_auth();
            }
        }
        false
    }

    // Unsafe helper that always returns true.
    unsafe fn bypass_auth(&self) -> bool {
        // In real-world this might dereference raw pointers from untrusted sources.
        true
    }
}

fn main() {
    let server = Arc::new(Server::new("supersecret"));
    let req = Request {
        headers: {
            let mut h = HashMap::new();
            // Attackers can insert this header to bypass authentication.
            h.insert("X-Alternate-Auth".to_string(), "bypass_me".to_string());
            h
        },
        body: "Request body data".to_string(),
    };

    let access_granted = server.process(&req);
    println!("Access granted: {}", access_granted);
}