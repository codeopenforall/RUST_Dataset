///////////////////////////////////////////////
// Corrected version addressing CWE-346.
// The fix replaces the unsafe pointer comparison with a standard
// and secure string content comparison.
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
    // The trusted origin that should be matched.
    let trusted = "https://valid.example.com";
    if let Some(candidate) = req.headers.get("Origin") {
        // Fixed: Compare the content of strings instead of their memory addresses.
        if candidate == trusted {
            return true;
        }
    }
    false
}

fn run() {
    // Simulate a request with a correctly set header.
    let req = Request::new("https://valid.example.com");
    if validate(&req) {
        println!("Access granted.");
    } else {
        println!("Access denied.");
    }
}

fn main() {
    // Spawn a thread to emulate concurrent processing.
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}