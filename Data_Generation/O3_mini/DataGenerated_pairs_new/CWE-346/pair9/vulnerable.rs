/* Vulnerable Version */
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Handler {
    // Expected host name, e.g., "trusted.com"
    config: String,
}

impl Handler {
    // Unsafe block used to simulate low‐level pointer operations.
    // Vulnerability: improper origin validation that relies on a substring check.
    // It constructs the expected URL as "https://{config}" and checks if the user‑supplied
    // Origin header contains that substring. An attacker can supply a header like 
    // "https://trusted.com@malicious.com" which passes the check.
    unsafe fn validate(&self, req: &Request) -> bool {
        let raw_ptr = req.headers.get("Origin").unwrap() as *const String;
        let origin = &*raw_ptr;
        // Construct the expected URL.
        let expected = format!("https://{}", self.config);
        // Vulnerable check: merely checks for substring presence.
        origin.contains(&expected)
    }

    fn process(&self, req: Request) -> bool {
        unsafe { self.validate(&req) }
    }
}

fn main() {
    // The configuration expects requests coming from "trusted.com"
    let handler = Arc::new(Handler { config: "trusted.com".to_string() });
    // Malicious request: The attacker sets the Origin header as:
    // "https://trusted.com@malicious.com" which contains "https://trusted.com" as a substring.
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://trusted.com@malicious.com".to_string());
    let req = Request { headers };
    let allowed = handler.process(req);
    println!("Request allowed: {}", allowed);

    // Spawn a thread for a benign request.
    let handler_clone = Arc::clone(&handler);
    let thread_handle = thread::spawn(move || {
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://trusted.com".to_string());
        let req = Request { headers };
        let allowed = handler_clone.process(req);
        println!("Thread request allowed: {}", allowed);
    });
    thread_handle.join().unwrap();
}