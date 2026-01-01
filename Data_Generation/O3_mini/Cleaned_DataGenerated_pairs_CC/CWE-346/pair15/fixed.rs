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
    approved: Arc<Vec<String>>,
}
impl Service {
    fn new(approved: Vec<String>) -> Self {
        Service {
            approved: Arc::new(approved),
        }
    }
    fn process(&self, req: Request) -> Result<String, &'static str> {
        let origin = req
            .headers
            .get("Origin")
            .ok_or("Missing Origin header")?;
        if !self.approved.contains(origin) {
            return Err("Origin not authorized");
        }
        let mut url = "http://".to_owned();
        url.push_str(origin);
        Ok(url)
    }
}
fn main() {
    let service = Service::new(vec!["trusted.com".to_string()]);
    let shared_service = Arc::new(service);
    let service_clone = Arc::clone(&shared_service);
    let handle = thread::spawn(move || {
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
