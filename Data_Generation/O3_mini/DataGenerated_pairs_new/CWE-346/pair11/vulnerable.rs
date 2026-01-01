//////////////////////////
// Vulnerable code sample
//////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

// A simple request structure holding header values.
struct Request {
    host: String,
    origin: String,
    referer: String,
}

// Global mutable counter updated in an unsafe way.
static mut REQUEST_COUNT: usize = 0;

// Parses a raw header string into a Request structure.
fn parse_headers(raw: &str) -> Request {
    let mut host = String::new();
    let mut origin = String::new();
    let mut referer = String::new();
    for line in raw.lines() {
        if let Some(idx) = line.find(':') {
            let (key, value) = line.split_at(idx);
            let value = value[1..].trim(); // skip ':' and trim
            match key.trim().to_lowercase().as_str() {
                "host" => host = value.to_string(),
                "origin" => origin = value.to_string(),
                "referer" => referer = value.to_string(),
                _ => {}
            }
        }
    }
    Request { host, origin, referer }
}

// UNSAFE header validation using a substring match against a trusted host.
// Vulnerability: by checking if the header contains the allowed domain as substring,
// an attacker may supply "trusted.example.evil.com" to bypass validations.
unsafe fn check_request(req: &Request) -> bool {
    // The trusted domain is hard-coded.
    let trusted = "trusted.example";
    if req.host.contains(trusted) {
        // Unsafe update to the global counter without synchronization.
        REQUEST_COUNT += 1;
        true
    } else {
        false
    }
}

// Core processing function for a request using unsafe header validation.
fn process(req: Request) -> Result<(), &'static str> {
    // Simulate concurrency by spawning a thread that does additional processing.
    let req_arc = Arc::new(Mutex::new(req));

    let req_clone = Arc::clone(&req_arc);
    let handle = thread::spawn(move || {
        let locked = req_clone.lock().unwrap();
        // Call the unsafe header check.
        unsafe { check_request(&locked) }
    });

    let valid = handle.join().unwrap();
    if valid {
        Ok(())
    } else {
        Err("Rejected header values")
    }
}

fn main() {
    // Example input with malicious host that includes the valid trusted domain as substring.
    // Vulnerability: "trusted.example.evil.com" passes the check.
    let raw = "Host: trusted.example.evil.com\nOrigin: http://malicious.example\nReferer: http://malicious.example";
    let request = parse_headers(raw);
    match process(request) {
        Ok(_) => println!("Request processed successfully."),
        Err(err) => println!("Error processing request: {}", err),
    }
}