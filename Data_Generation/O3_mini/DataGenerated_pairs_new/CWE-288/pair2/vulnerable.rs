//////////////////////////////////////////////
// Vulnerable Rust Code Example (CWE-288)
// This example simulates a server with two endpoints.
// The debug endpoint mistakenly bypasses authentication
// by relying on an unsafe global flag.
//////////////////////////////////////////////

use std::sync::{Arc, Mutex};

static mut GLOBAL_DEBUG: bool = true; // Unsafe global flag

// Represents an HTTP-like request.
struct Request {
    endpoint: String,
    token: Option<String>,
}

// Server configuration structure.
struct Server {
    secret: String, // Expected authentication token.
    config: Arc<Mutex<Config>>,
}

// Configuration controlling auth requirements.
struct Config {
    require_auth: bool,
}

// Contract for processing requests.
trait Processor {
    fn process_request(&self, req: &Request) -> bool;
}

impl Server {
    fn new(secret: &str, require_auth: bool) -> Self {
        Server {
            secret: secret.to_string(),
            config: Arc::new(Mutex::new(Config { require_auth })),
        }
    }
}

impl Processor for Server {
    fn process_request(&self, req: &Request) -> bool {
        // Normal secure administration endpoint.
        if req.endpoint == "/admin" {
            let cfg = self.config.lock().unwrap();
            if cfg.require_auth {
                return req.token.as_ref() == Some(&self.secret);
            }
            return true;
        }
        // Debug endpoint that incorrectly bypasses authentication.
        else if req.endpoint == "/debug" {
            // Vulnerable block: using unsafe global bypass flag.
            unsafe {
                if GLOBAL_DEBUG {
                    // Bypassing token check!
                    return true;
                }
            }
            // Fallback check (never reached if GLOBAL_DEBUG is true).
            return req.token.as_ref() == Some(&self.secret);
        }
        false
    }
}

fn main() {
    let server = Server::new("supersecret", true);
    // A valid admin request with proper token.
    let req_admin = Request {
        endpoint: "/admin".to_string(),
        token: Some("supersecret".to_string()),
    };
    // A debug endpoint request missing token (bypasses auth in vulnerable version).
    let req_debug = Request {
        endpoint: "/debug".to_string(),
        token: None,
    };
    println!("Admin endpoint access: {}", server.process_request(&req_admin));
    println!("Debug endpoint access: {}", server.process_request(&req_debug));
}