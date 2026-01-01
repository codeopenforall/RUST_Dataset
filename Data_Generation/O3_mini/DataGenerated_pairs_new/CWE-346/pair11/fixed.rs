//////////////////////////
// Corrected code sample
//////////////////////////
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

// A simple request structure holding header values.
struct Request {
    host: String,
    origin: String,
    referer: String,
}

// Thread-safe global request counter.
static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

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

// Thread-safe header validation with strict equality check.
// The host header must exactly match the trusted domain.
fn check_request(req: &Request) -> bool {
    let trusted = "trusted.example";
    if req.host == trusted {
        // Safely update the atomic global counter.
        REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);
        true
    } else {
        false
    }
}

// Core processing function for a request using secure validation.
fn process(req: Request) -> Result<(), &'static str> {
    // Simulate concurrency by spawning a thread for processing.
    let req_arc = Arc::new(Mutex::new(req));

    let req_clone = Arc::clone(&req_arc);
    let handle = thread::spawn(move || {
        let locked = req_clone.lock().unwrap();
        check_request(&locked)
    });

    let valid = handle.join().unwrap();
    if valid {
        Ok(())
    } else {
        Err("Rejected header values")
    }
}

fn main() {
    // Example input where the Host header must exactly equal "trusted.example".
    // Malicious header "trusted.example.evil.com" is rejected.
    let raw = "Host: trusted.example.evil.com\nOrigin: http://malicious.example\nReferer: http://malicious.example";
    let request = parse_headers(raw);
    match process(request) {
        Ok(_) => println!("Request processed successfully."),
        Err(err) => println!("Error processing request: {}", err),
    }
}