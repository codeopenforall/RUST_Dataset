////////////////////////////////////////////////////////////
// Vulnerable Code Sample
////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new(headers: HashMap<String, String>, body: String) -> Self {
        Request { headers, body }
    }
}

struct Service {
    // A list of allowed origins, but not used in validation below.
    approved: Arc<Vec<String>>,
}

impl Service {
    fn new(approved: Vec<String>) -> Self {
        Service {
            approved: Arc::new(approved),
        }
    }

    // Vulnerable processing: it trusts the header without checking.
    fn process(&self, req: Request) -> Result<String, &'static str> {
        // Lazy retrieval of the Origin header; if missing, error out.
        let origin = req
            .headers
            .get("Origin")
            .ok_or("Missing Origin header")?;

        // UNSAFE: Converting fixed byte array to a string without checking.
        // Also, the origin string from the user is concatenated without validation,
        // leading to a potential SSRF or CSRF pivot.
        let mut url = unsafe {
            // Arbitrarily create a string starting with "http"
            String::from_utf8_unchecked(vec![b'h', b't', b't', b'p'])
        };
        url.push_str("://");
        url.push_str(origin);
        Ok(url)
    }
}

fn main() {
    // Simulate a concurrent environment.
    let service = Service::new(vec!["trusted.com".to_string()]);
    let shared_service = Arc::new(service);

    let service_clone = Arc::clone(&shared_service);
    let handle = thread::spawn(move || {
        // A request with a malicious Origin header is processed.
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "malicious.com".to_string());
        let req = Request::new(headers, String::new());
        match service_clone.process(req) {
            Ok(url) => println!("Constructed URL: {}", url),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

    handle.join().unwrap();
}