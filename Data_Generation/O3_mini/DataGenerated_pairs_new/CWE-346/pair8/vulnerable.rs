/////////////////////////////////////////////////////////////////
// Vulnerable Code -- CWE-346: Origin Validation Error
// This code simulates an HTTP-like request processing wherein
// the origin (host) header is validated using an unsafe pointer‐
// equality check. The check compares the pointer of the header’s
// string to the pointer of a string literal "trusted.example.com".
// This approach is flawed: if the header value is constructed at run‑
// time (even with equivalent content), its pointer will differ,
// causing validation to fail. In real-world scenarios, such mistakes
// could lead to CSRF/SSRF vulnerabilities if the logic were inverted
// or misapplied.
/////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Request {
    headers: HashMap<String, String>,
}

impl Request {
    fn new() -> Self {
        Request {
            headers: HashMap::new(),
        }
    }
}

// Unsafe validation function that erroneously compares memory addresses.
unsafe fn check_origin(req: &Request) -> bool {
    if let Some(val) = req.headers.get("host") {
        // Vulnerable: comparing pointers instead of full content.
        // If the header is a dynamically allocated String (even if its content
        // equals the literal), the pointer comparison will fail.
        if std::ptr::eq(val.as_str(), "trusted.example.com") {
            return true;
        }
    }
    false
}

fn process_request(req: &Request) -> bool {
    // In a simulated concurrent scenario, we wrap the unsafe call.
    unsafe { check_origin(req) }
}

fn main() {
    // Simulating multiple concurrent request processing threads.
    let req = {
        let mut r = Request::new();
        // Intentionally constructing the header value dynamically.
        // Although the content matches "trusted.example.com", its pointer
        // is different from that of the literal in check_origin.
        r.headers.insert("host".to_string(), String::from("trusted.example.com"));
        r
    };

    let handler = thread::spawn(move || {
        // Simulate some delay.
        thread::sleep(Duration::from_millis(50));
        let valid = process_request(&req);
        if valid {
            println!("Access granted.");
        } else {
            println!("Access denied.");
        }
        valid
    });

    // Wait for thread to complete.
    let res = handler.join().unwrap();
    // For demonstration, also exit with appropriate status.
    if res {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}