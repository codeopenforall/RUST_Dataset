//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc};
use std::thread;
use std::ffi::CStr;

/// A simple HTTP request representation.
pub struct Request {
    pub headers: HashMap<String, String>,
}

impl Request {
    /// Checks if the "Origin" header matches a trusted value.
    /// 
    /// Vulnerability Details:
    /// The code unsafely converts a Rust String into a C string assuming it is null-terminated.
    /// An attacker can embed a null byte (\\0) in the header value, causing the CStr conversion
    /// to truncate the string and mistakenly accept a malicious header.
    pub fn check_origin(&self) -> bool {
        // Trusted origin value
        let trusted = "https://example.com";
        if let Some(origin) = self.headers.get("Origin") {
            unsafe {
                // UNSAFE: Assumes "origin" is a null-terminated C string.
                let ptr = origin.as_ptr() as *const i8;
                // This conversion will stop at the first null byte.
                // An attacker may inject a null byte to bypass the full check.
                let cstr_origin = CStr::from_ptr(ptr);
                match cstr_origin.to_str() {
                    Ok(s) => s == trusted,
                    Err(_) => false,
                }
            }
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
                // Process request if origin is allowed.
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
    // Malicious input: embeds a null byte to trick the unsafe CStr conversion.
    let mut hdrs = HashMap::new();
    hdrs.insert("Origin".to_string(), "https://example.com\0.malicious".to_string());
    let req = Request { headers: hdrs };
    process_request(req);
}