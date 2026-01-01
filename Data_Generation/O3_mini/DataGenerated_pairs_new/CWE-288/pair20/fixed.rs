//////////////////////////////
// Fixed Code Sample
//////////////////////////////
use std::slice;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

struct Request {
    headers: Vec<(String, String)>,
}

impl Request {
    // Still using unsafe conversion; assumes headers are generated internally.
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

    // Corrected authentication routine.
    // Even if debug is enabled, requests must provide either a valid token
    // or a valid debug-origin header to be authenticated.
    fn authenticate(&self, req: &Request) -> bool {
        unsafe {
            if let Some(token) = req.get_header_unchecked("Authorization") {
                if token == self.expected_token {
                    return true;
                }
            }
        }
        // In debug mode, allow bypass only if the request explicitly indicates a trusted origin.
        if self.debug_enabled {
            unsafe {
                if let Some(origin) = req.get_header_unchecked("X-Origin") {
                    if origin == "localhost" {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    // Simulated concurrent request handling.
    fn serve_request(&self, req: Request) -> bool {
        let auth_result = Arc::new(Mutex::new(false));
        let auth_clone = Arc::clone(&auth_result);
        let req_clone = Request {
            headers: req.headers.clone(),
        };
        let token = self.expected_token.clone();
        let debug = self.debug_enabled;
        let handle = thread::spawn(move || {
            let mut result = false;
            unsafe {
                if let Some(token_val) = req_clone.get_header_unchecked("Authorization") {
                    if token_val == token {
                        result = true;
                    }
                }
                if !result && debug {
                    if let Some(origin) = req_clone.get_header_unchecked("X-Origin") {
                        if origin == "localhost" {
                            result = true;
                        }
                    }
                }
            }
            let mut guard = auth_clone.lock().unwrap();
            *guard = result;
        });
        handle.join().unwrap();
        let guard = auth_result.lock().unwrap();
        *guard
    }
}

fn main() {
    // Production-like simulation:
    // The request is missing a valid Authorization header and lacks a trusted origin.
    // With the fix, authentication will correctly deny access.
    let req = Request {
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            // Note: "X-Origin" is set to an untrusted host.
            ("X-Origin".to_string(), "production_host".to_string())
        ],
    };
    let handler = Handler::new(true, "secret");
    let access = handler.serve_request(req);
    println!("Access granted: {}", access);
}