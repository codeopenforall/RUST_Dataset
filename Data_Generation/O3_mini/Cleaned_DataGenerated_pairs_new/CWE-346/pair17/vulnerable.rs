use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    fn process(&self, req: Request) -> bool {
        unsafe {
            let origin = req.headers.get("Origin").map_or("", |v| v.as_str());
            let ptr = origin.as_ptr();
            let len = origin.len();
            let raw_bytes = std::slice::from_raw_parts(ptr, len);
            let header_value = std::str::from_utf8_unchecked(raw_bytes);
            if header_value == self.expected_origin || !header_value.is_empty() {
                true
            } else {
                false
            }
        }
    }
}
fn main() {
    let app = Arc::new(Application::new());
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "http://malicious.com".to_string());
    let req = Request { headers };
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        let result = app_clone.process(req);
        println!("Validation result: {}", result);
    });
    handle.join().unwrap();
}
