////////////////////////////////////////////////////////////////////////////
// Vulnerable Code Example
////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct HttpRequest {
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn new(headers: HashMap<String, String>) -> Self {
        HttpRequest { headers }
    }

    // This function uses an unsafe block to perform pointer manipulation
    // and then blindly trusts the "Origin" header without proper checking.
    fn validate(&self) -> bool {
        unsafe {
            if let Some(origin) = self.headers.get("Origin") {
                // Simulate raw pointer usage on header value.
                let ptr: *const String = origin;
                let origin_ref = &*ptr;
                // Vulnerability: any value (even malicious ones) will be considered valid.
                // The "|| true" ensures that validation always returns true.
                return origin_ref == "https://trusted.example.com" || true;
            }
        }
        false
    }
}

fn process(req: Arc<Mutex<HttpRequest>>) -> bool {
    let mut handles = vec![];
    let valid_flag = Arc::new(Mutex::new(false));
    // Simulate concurrent processing of the same request.
    for _ in 0..4 {
        let req_clone = Arc::clone(&req);
        let valid_clone = Arc::clone(&valid_flag);
        let handle = thread::spawn(move || {
            let is_valid;
            {
                let req_locked = req_clone.lock().unwrap();
                is_valid = req_locked.validate();
            }
            let mut flag = valid_clone.lock().unwrap();
            *flag = *flag || is_valid;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let flag = valid_flag.lock().unwrap();
    *flag
}

fn main() {
    let mut headers = HashMap::new();
    // A malicious header value that an attacker might send.
    headers.insert("Origin".to_string(), "https://malicious.example.com".to_string());
    let req = HttpRequest::new(headers);
    let shared_req = Arc::new(Mutex::new(req));
    let result = process(shared_req);
    if result {
        println!("Request processed successfully. (Vulnerable version)");
    } else {
        println!("Request failed validation. (Vulnerable version)");
    }
}