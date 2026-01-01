//////////////////////////////////////////////////////////////////
// Corrected Code - Strict Origin Verification without Unsafe Use
//////////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
}

struct Application {
    expected_origin: &'static str,
}

impl Application {
    fn new() -> Self {
        Application {
            expected_origin: "https://trusted.com",
        }
    }
    
    // Securely validates the origin header.
    // Only an exact match with the expected origin is considered valid.
    fn process(&self, req: Request) -> bool {
        // Safely extract the Origin header.
        let origin = req.headers.get("Origin").map_or("", |v| v.as_str());
        // Enforce strict equality for origin validation.
        origin == self.expected_origin
    }
}

fn main() {
    let app = Arc::new(Application::new());
    let mut headers = std::collections::HashMap::new();
    // A malicious header value that should now fail the strict check.
    headers.insert("Origin".to_string(), "http://malicious.com".to_string());
    let req = Request { headers };

    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        let result = app_clone.process(req);
        println!("Validation result: {}", result);
    });
    handle.join().unwrap();
}