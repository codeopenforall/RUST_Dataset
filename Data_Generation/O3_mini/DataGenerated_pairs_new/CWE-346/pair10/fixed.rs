//////////////// Fixed Code ////////////////////
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    allowed_origin: String,
}

impl Server {
    // Correctly processes the request by safely validating the header value.
    fn process(&self, req: &Request) -> bool {
        // Safely retrieve the "Origin" header.
        let origin = match req.headers.get("Origin") {
            Some(value) => value,
            None => return false,
        };

        // Validate the header value format and compare against the allowed origin.
        if origin.starts_with("https://") && origin == &self.allowed_origin {
            true
        } else {
            false
        }
    }
}

fn main() {
    let server = Arc::new(Server { allowed_origin: "https://secure.example.com".to_string() });

    let req = Request {
        headers: {
            let mut h = HashMap::new();
            // Provide a properly validated header.
            h.insert("Origin".to_string(), "https://secure.example.com".to_string());
            h
        },
        body: "Test body".to_string(),
    };

    let server_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        let allowed = server_clone.process(&req);
        if allowed {
            println!("Request processed safely.");
        } else {
            println!("Request rejected.");
        }
    });

    handle.join().unwrap();
}