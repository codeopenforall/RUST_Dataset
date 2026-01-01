use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Engine;

impl Engine {
    fn run(&self, req: Arc<Request>) -> bool {
        let shared_req = req.clone();
        // Spawn a thread to simulate concurrent handling.
        let handle = thread::spawn(move || {
            // Proper header validation.
            validate_header(&shared_req.headers)
        });
        handle.join().unwrap()
    }
}

fn validate_header(headers: &HashMap<String, String>) -> bool {
    let anchor = "https://trusted.com";
    if let Some(value) = headers.get("Origin") {
        // Correctly compare the entire header value.
        value == anchor
    } else {
        false
    }
}

fn main() {
    // Build a simulated request with an Origin header.
    let mut hdrs = HashMap::new();
    // Malicious header: additional data appended to a valid origin.
    hdrs.insert("Origin".to_string(), "https://trusted.com.evil".to_string());
    let request = Arc::new(Request { headers: hdrs });
    let service = Engine;
    let outcome = service.run(request);
    if outcome {
        println!("Request passed validation");
    } else {
        println!("Request failed validation");
    }
}