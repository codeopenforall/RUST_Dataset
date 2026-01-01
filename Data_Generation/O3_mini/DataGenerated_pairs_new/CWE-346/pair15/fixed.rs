////////////////////////////////////////////////////////////
// Corrected Code Sample
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
    // A list of allowed origins.
    approved: Arc<Vec<String>>,
}

impl Service {
    fn new(approved: Vec<String>) -> Self {
        Service {
            approved: Arc::new(approved),
        }
    }

    // Correct processing: validates the origin against an approved list.
    fn process(&self, req: Request) -> Result<String, &'static str> {
        let origin = req
            .headers
            .get("Origin")
            .ok_or("Missing Origin header")?;

        // Check the Origin header against the whitelist.
        if !self.approved.contains(origin) {
            return Err("Origin not authorized");
        }
        // Use safe string operations to build the URL.
        let mut url = "http://".to_owned();
        url.push_str(origin);
        Ok(url)
    }
}

fn main() {
    // Simulate a concurrent processing environment.
    let service = Service::new(vec!["trusted.com".to_string()]);
    let shared_service = Arc::new(service);

    let service_clone = Arc::clone(&shared_service);
    let handle = thread::spawn(move || {
        // A request with a non-approved Origin header triggers the validation.
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