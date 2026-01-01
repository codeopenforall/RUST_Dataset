/*
This Rust program handles requests securely by properly validating the Origin header.
The fix explicitly compares the header string against the trusted origin ("http://example.com").
It nevertheless spawns several threads for concurrent processing, following community idioms.
Compile with: rustc fixed.rs
*/
use std::thread;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
struct Request {
    origin: String,
    path: String,
}

/// Securely validates the Origin header by comparing it to the trusted origin.
fn validate_origin(request: &Request) -> bool {
    // Only accept the request if the header exactly matches the trusted value.
    request.origin == "http://example.com"
}

/// Processes the incoming request concurrently by spawning multiple threads to validate it.
fn process_request(request: Request) -> &'static str {
    let req_arc = Arc::new(request);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let req_clone = req_arc.clone();
        handles.push(thread::spawn(move || {
            validate_origin(&req_clone)
        }));
    }
    for h in handles {
        // Reject the request if any thread finds the header invalid.
        if !h.join().unwrap() {
            return "Rejected";
        }
    }
    "Accepted"
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // If no argument is provided, use a default malicious header.
    let header = if args.len() > 1 {
        args[1].clone()
    } else {
        "http://evil.com".to_string()
    };
    let req = Request {
        origin: header,
        path: "/".to_string(),
    };
    let result = process_request(req);
    println!("{}", result);
}