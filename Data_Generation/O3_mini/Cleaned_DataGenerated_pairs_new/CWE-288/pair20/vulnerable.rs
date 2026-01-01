use std::slice;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
struct Request {
    headers: Vec<(String, String)>,
}
impl Request {
    unsafe fn get_header_unchecked(&self, name: &str) -> Option<&str> {
        for (k, v) in &self.headers {
            if k == name {
                let ptr = v.as_ptr();
                let len = v.len();
                let slice = slice::from_raw_parts(ptr, len);
                return Some(str::from_utf8_unchecked(slice));
            }
        }
        None
    }
}
struct Handler {
    debug_enabled: bool,
    expected_token: String,
}
impl Handler {
    fn new(debug_enabled: bool, expected_token: &str) -> Self {
        Handler {
            debug_enabled,
            expected_token: expected_token.to_string(),
        }
    }
    fn authenticate(&self, req: &Request) -> bool {
        if self.debug_enabled {
            return true;
        }
        unsafe {
            if let Some(token) = req.get_header_unchecked("Authorization") {
                if token == self.expected_token {
                    return true;
                }
            }
        }
        false
    }
    fn serve_request(&self, req: Request) -> bool {
        let auth_result = Arc::new(Mutex::new(false));
        let auth_clone = Arc::clone(&auth_result);
        let req_clone = Request {
            headers: req.headers.clone(),
        };
        let token = self.expected_token.clone();
        let debug = self.debug_enabled;
        let handle = thread::spawn(move || {
            let result = if debug {
                true
            } else {
                unsafe {
                    if let Some(t) = req_clone.get_header_unchecked("Authorization") {
                        t == token
                    } else {
                        false
                    }
                }
            };
            let mut guard = auth_clone.lock().unwrap();
            *guard = result;
        });
        handle.join().unwrap();
        let guard = auth_result.lock().unwrap();
        *guard
    }
}
fn main() {
    let req = Request {
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string())
        ],
    };
    let handler = Handler::new(true, "secret");
    let access = handler.serve_request(req);
    println!("Access granted: {}", access);
}
