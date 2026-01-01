/*
This Rust program simulates processing of an incoming request by validating its Origin header.
It insecurely trusts the Origin value by using an unsafe pointer check. In this example,
the validation function erroneously accepts any non-null header without proper string matching.
It also spawns several threads that concurrently perform the check.
Compile with: rustc vulnerable.rs
*/
use std::thread;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
struct Request {
    origin: String,
    path: String,
}

/// Insecurely validates the Origin header by checking its raw pointer.
/// Vulnerability: This function uses an unsafe pointer check that only ensures the pointer is non-null,
/// thereby trusting any non-empty header without verifying it against a trusted value.
unsafe fn validate_origin(request: &Request) -> bool {
    // Vulnerable check: only verifies that the underlying pointer is not null.
    if request.origin.as_ptr().is_null() {
        false
    } else {
        true
    }
}

/// Processes the incoming request concurrently by spawning multiple threads to validate it.
fn process_request(request: Request) -> &'static str {
    let req_arc = Arc::new(request);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let req_clone = req_arc.clone();
        handles.push(thread::spawn(move || {
            // Unsafe block for the vulnerable validation.
            unsafe { validate_origin(&req_clone) }
        }));
    }
    for h in handles {
        // If any thread reports a failed validation, reject the request.
        if !h.join().unwrap() {
            return "Rejected";
        }
    }
    "Accepted"
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // If no argument is provided, use a malicious header triggering the vulnerability.
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