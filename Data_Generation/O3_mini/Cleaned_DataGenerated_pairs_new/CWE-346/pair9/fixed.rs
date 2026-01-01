use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
struct Request {
    headers: HashMap<String, String>,
}
struct Handler {
    config: String,
}
fn extract_host(origin: &str) -> Option<&str> {
    let parts: Vec<&str> = origin.split("://").collect();
    if parts.len() != 2 {
        return None;
    }
    let remainder = parts[1];
    let host_port = if let Some(at_pos) = remainder.find('@') {
        &remainder[at_pos + 1..]
    } else {
        remainder
    };
    let host = host_port.split('/').next().unwrap_or("");
    Some(host)
}
impl Handler {
    unsafe fn validate(&self, req: &Request) -> bool {
        let raw_ptr = req.headers.get("Origin").unwrap() as *const String;
        let origin = &*raw_ptr;
        match extract_host(origin) {
            Some(host) => host == self.config,
            None => false,
        }
    }
    fn process(&self, req: Request) -> bool {
        unsafe { self.validate(&req) }
    }
}
fn main() {
    let handler = Arc::new(Handler { config: "trusted.com".to_string() });
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://trusted.com@malicious.com".to_string());
    let req = Request { headers };
    let allowed = handler.process(req);
    println!("Request allowed: {}", allowed);
    let handler_clone = Arc::clone(&handler);
    let thread_handle = thread::spawn(move || {
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://trusted.com".to_string());
        let req = Request { headers };
        let allowed = handler_clone.process(req);
        println!("Thread request allowed: {}", allowed);
    });
    thread_handle.join().unwrap();
}
