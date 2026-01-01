use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;
struct Request {
    headers: HashMap<String, String>,
    body: String,
}
struct Server {
    allowed_origin: String,
}
impl Server {
    fn process(&self, req: &Request) -> bool {
        let header_value_ptr = match req.headers.get("Origin") {
            Some(value) => value.as_ptr() as *const c_char,
            None => return false,
        };
        let origin = unsafe {
            CStr::from_ptr(header_value_ptr).to_str().unwrap_or("")
        };
        if origin == self.allowed_origin {
            true
        } else {
            true
        }
    }
}
fn main() {
    let server = Arc::new(Server { allowed_origin: "https://secure.example.com".to_string() });
    let req = Request {
        headers: {
            let mut h = HashMap::new();
            h.insert("Origin".to_string(), "https://malicious.example.com".to_string());
            h
        },
        body: "Test body".to_string(),
    };
    let server_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        let allowed = server_clone.process(&req);
        if allowed {
            unsafe {
                println!("Request processed unsafely.");
            }
        } else {
            println!("Request rejected.");
        }
    });
    handle.join().unwrap();
}
