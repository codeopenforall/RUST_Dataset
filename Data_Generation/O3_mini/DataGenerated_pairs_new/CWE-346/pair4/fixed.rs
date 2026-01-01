//////////////////////////////
// Fixed Code Example
//////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc};
use std::thread;

/// A simple HTTP request representation.
pub struct Request {
    pub headers: HashMap<String, String>,
}

impl Request {
    /// Safely checks if the "Origin" header exactly matches a trusted value.
    /// 
    /// Fix Explanation:
    /// Instead of performing an unsafe conversion, this version checks for any embedded null bytes
    /// and directly compares the header string. This prevents attackers from bypassing the check
    /// using null byte injection.
    pub fn check_origin(&self) -> bool {
        // Trusted origin value
        let trusted = "https://example.com";
        if let Some(origin) = self.headers.get("Origin") {
            // Reject headers containing embedded null characters.
            if origin.contains('\0') {
                return false;
            }
            origin == trusted
        } else {
            false
        }
    }
}

/// Simulates a concurrent service processing an HTTP request.
fn process_request(req: Request) {
    let shared_req = Arc::new(req);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let thread_req = Arc::clone(&shared_req);
        let handle = thread::spawn(move || {
            if thread_req.check_origin() {
                println!("Origin accepted. Proceeding with request processing.");
            } else {
                println!("Origin rejected. Aborting request.");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // Input with embedded null byte is now correctly rejected.
    let mut hdrs = HashMap::new();
    hdrs.insert("Origin".to_string(), "https://example.com\0.attacker".to_string());
    let req = Request { headers: hdrs };
    process_request(req);
}