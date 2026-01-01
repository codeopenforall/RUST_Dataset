////////////////////////////////////
// Vulnerable Code Example        //
// CWE-346: Origin Validation Error//
// This code trusts header values   //
// from the client without strict   //
// validation and uses unsafe blocks//
////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct HttpRequest {
    headers: Vec<(String, String)>,
    payload: String,
}

impl HttpRequest {
    fn new(headers: Vec<(String, String)>, payload: String) -> Self {
        HttpRequest { headers, payload }
    }
}

struct Server {
    allowed: Arc<Mutex<Vec<String>>>,
}

impl Server {
    fn new() -> Self {
        // The allowed list only contains the trusted origin.
        Server { allowed: Arc::new(Mutex::new(vec!["https://example.com".to_string()])) }
    }

    // This function unsafely inspects header data and performs a weak check.
    // It mistakenly trusts any origin header which merely contains "example.com".
    fn trust_header(&self, req: &HttpRequest) -> bool {
        // UNSAFE: We use a raw pointer to iterate the vector.
        unsafe {
            let headers_ptr = req.headers.as_ptr();
            for i in 0..req.headers.len() {
                // Read header entry from raw memory.
                let header = headers_ptr.add(i).read();
                if header.0 == "Origin" {
                    // Vulnerability: instead of an exact match, a substring check is used.
                    if header.1.contains("example.com") {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn run(&self, req: HttpRequest) -> &'static str {
        if self.trust_header(&req) {
            "Access granted"
        } else {
            "Access denied"
        }
    }
}

fn main() {
    let server = Server::new();
    // A malicious request crafted to bypass origin validation.
    let req = HttpRequest::new(
        vec![("Origin".to_string(), "http://malicious.example.com".to_string())],
        "sensitive payload".to_string(),
    );

    let server_arc = Arc::new(server);
    let mut handles = vec![];

    for _ in 0..4 {
        let srv = Arc::clone(&server_arc);
        let req_clone = HttpRequest {
            headers: req.headers.clone(),
            payload: req.payload.clone(),
        };
        handles.push(thread::spawn(move || {
            println!("{}", srv.run(req_clone));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}