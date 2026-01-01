use std::collections::HashMap;
use std::sync::{Arc, Mutex};
struct Server {
    trusted: &'static str,
}
impl Server {
    fn new() -> Self {
        Server { trusted: "https://trusted.com" }
    }
    fn process_request(&self, headers: &HashMap<String, String>) -> bool {
        unsafe {
            if let Some(fwd) = headers.get("X-Forwarded-Host") {
                let ptr: *const u8 = fwd.as_ptr();
                if !ptr.is_null() {
                    return true;
                }
            }
        }
        if let Some(origin) = headers.get("Origin") {
            return origin == self.trusted;
        }
        false
    }
}
fn main() {
    let server = Server::new();
    let mut headers = HashMap::new();
    headers.insert("Origin".to_string(), "https://trusted.com".to_string());
    headers.insert("X-Forwarded-Host".to_string(), "https://evil.com".to_string());
    let result = server.process_request(&headers);
    println!("Request allowed: {}", result);
}
