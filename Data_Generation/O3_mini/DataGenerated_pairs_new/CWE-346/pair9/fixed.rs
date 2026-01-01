/* Fixed Version */
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

// A helper function that extracts the host from an origin string.
// It splits the input on "://" and then, if present, removes any userinfo part before '@'.
// For example, "https://trusted.com@malicious.com" yields host "malicious.com".
fn extract_host(origin: &str) -> Option<&str> {
    let parts: Vec<&str> = origin.split("://").collect();
    if parts.len() != 2 {
        return None;
    }
    let remainder = parts[1];
    // If there is userinfo (detected by '@'), take substring after '@'
    let host_port = if let Some(at_pos) = remainder.find('@') {
        &remainder[at_pos + 1..]
    } else {
        remainder
    };
    // Remove any path or port information.
    let host = host_port.split('/').next().unwrap_or("");
    Some(host)
}

impl Handler {
    // Unsafe is still used to mimic similar low-level operations,
    // but the validation properly extracts and compares the host.
    unsafe fn validate(&self, req: &Request) -> bool {
        let raw_ptr = req.headers.get("Origin").unwrap() as *const String;
        let origin = &*raw_ptr;
        match extract_host(origin) {
            Some(host) => host == self.config,
            None => false,
        }
    }

    fn process(&self, req: Request) -> bool {
        unsafe { self.validate(&req) }
    }
}

fn main() {
    // The configuration expects the host to be "trusted.com"
    let handler = Arc::new(Handler { config: "trusted.com".to_string() });
    // Malicious request: Even if the header is "https://trusted.com@malicious.com",
    // the host extracted will be "malicious.com" and will not match "trusted.com".
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