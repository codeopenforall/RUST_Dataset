//////////////////////////////////////////////
// Fixed Rust Code Example (CWE-288)
// This corrected version removes the unsafe bypass path for the debug route,
// enforcing proper authentication for all endpoints.
//////////////////////////////////////////////

use std::sync::{Arc, Mutex};

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

// Configuration controlling authentication requirements.
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
        // Both endpoints now require proper authentication.
        if req.endpoint == "/admin" || req.endpoint == "/debug" {
            let cfg = self.config.lock().unwrap();
            if cfg.require_auth {
                return req.token.as_ref() == Some(&self.secret);
            }
            return true;
        }
        false
    }
}

fn main() {
    let server = Server::new("supersecret", true);
    // A valid admin request with the correct token.
    let req_admin = Request {
        endpoint: "/admin".to_string(),
        token: Some("supersecret".to_string()),
    };
    // Debug endpoint now also requires a correct token.
    let req_debug = Request {
        endpoint: "/debug".to_string(),
        token: Some("supersecret".to_string()),
    };
    println!("Admin endpoint access: {}", server.process_request(&req_admin));
    println!("Debug endpoint access: {}", server.process_request(&req_debug));
}