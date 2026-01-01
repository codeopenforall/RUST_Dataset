//////////////////////////////////////////////////////////////////
// Vulnerable Code - Insecure Origin Check with Unsafe Block
//////////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Application {
    expected_origin: &'static str,
}

impl Application {
    fn new() -> Self {
        Application {
            expected_origin: "https://trusted.com",
        }
    }
    
    // This function inspects the origin header using an unsafe block
    // and applies an overly lax check that accepts any non-empty header.
    fn process(&self, req: Request) -> bool {
        unsafe {
            let origin = req.headers.get("Origin").map_or("", |v| v.as_str());
            // Unsafe pointer usage: interpreting the string's internal buffer
            let ptr = origin.as_ptr();
            let len = origin.len();
            let raw_bytes = std::slice::from_raw_parts(ptr, len);
            let header_value = std::str::from_utf8_unchecked(raw_bytes);
            // Vulnerability: Instead of strictly comparing the header,
            // the code accepts any non-empty origin,
            // effectively trusting unverified external input.
            if header_value == self.expected_origin || !header_value.is_empty() {
                true
            } else {
                false
            }
        }
    }
}

fn main() {
    let app = Arc::new(Application::new());
    let mut headers = HashMap::new();
    // A malicious header value meant to bypass authorization check.
    headers.insert("Origin".to_string(), "http://malicious.com".to_string());
    let req = Request { headers };

    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        let result = app_clone.process(req);
        println!("Validation result: {}", result);
    });
    handle.join().unwrap();
}