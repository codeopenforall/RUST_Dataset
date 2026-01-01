///////////////////////////////////////////////
// Vulnerability demonstration in Rust (CWE-346)
// This code simulates an origin header check where the unsafe block
// incorrectly compares string pointers rather than string contents.
// An attacker might craft input that subverts this check.
///////////////////////////////////////////////

use std::collections::HashMap;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

impl Request {
    fn new(origin: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Origin"), origin.to_string());
        Request { headers }
    }
}

fn validate(req: &Request) -> bool {
    // The trusted origin string literal.
    let trusted = "https://valid.example.com";
    if let Some(candidate) = req.headers.get("Origin") {
        unsafe {
            // Vulnerability: comparing the pointers rather than the string contents.
            // This check will pass only if both strings share the same memory location.
            let cand_ptr = candidate.as_ptr();
            let trusted_ptr = trusted.as_ptr();
            if cand_ptr == trusted_ptr {
                return true;
            }
        }
    }
    false
}

fn run() {
    // Simulate receiving a request that appears to have a valid header.
    let req = Request::new("https://valid.example.com");
    if validate(&req) {
        println!("Access granted.");
    } else {
        println!("Access denied.");
    }
}

fn main() {
    // Using a separate thread to simulate concurrent processing.
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}