/////////////////////////////////////////////////////////////////
// Corrected Code -- CWE-346: Origin Validation Error
// This corrected code properly validates the incoming origin header
// by comparing the string content (and not just raw pointers) in a safe
// manner. It adheres to proper practices by using standard equality checks
// and preserving the developer’s intent. Additionally, the code maintains
// concurrency for realistic request handling.
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

fn check_origin(req: &Request) -> bool {
    // Correct: Use safe string equality.
    if let Some(val) = req.headers.get("host") {
        if val == "trusted.example.com" {
            return true;
        }
    }
    false
}

fn process_request(req: &Request) -> bool {
    check_origin(req)
}

fn main() {
    // Simulating multiple concurrent request processing threads.
    let req = {
        let mut r = Request::new();
        // Construct header value dynamically.
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
    if res {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}