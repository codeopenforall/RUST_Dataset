////////////////////////////////////
// Corrected Code Example         //
// CWE-346: Origin Validation Fix   //
// This code now validates headers  //
// properly by checking for an exact //
// match against an allowed list.   //
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
        Server {
            // Only an exact trusted origin is allowed.
            allowed: Arc::new(Mutex::new(vec!["https://example.com".to_string()])),
        }
    }

    // This function safely iterates through the headers and checks if the "Origin"
    // header exactly matches one of the allowed origins.
    fn verify_header(&self, req: &HttpRequest) -> bool {
        let allowed = self.allowed.lock().unwrap();
        for (key, value) in &req.headers {
            if key == "Origin" && allowed.contains(value) {
                return true;
            }
        }
        false
    }

    fn run(&self, req: HttpRequest) -> &'static str {
        if self.verify_header(&req) {
            "Access granted"
        } else {
            "Access denied"
        }
    }
}

fn main() {
    let server = Server::new();
    // A malicious request that should now be rejected.
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