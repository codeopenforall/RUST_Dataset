//////////////////////////////////////
// Vulnerable implementation (unsafe)
//////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// Global unsynchronized mutable state (unsafe)
static mut GLOBAL_CONTENT_LENGTH: usize = 0;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub trait HttpParser {
    fn parse(req: &str) -> Result<HttpRequest, &'static str>;
}

pub struct ParserImpl;

impl HttpParser for ParserImpl {
    fn parse(req: &str) -> Result<HttpRequest, &'static str> {
        // Split request using a simple CRLF double separator; may fail to catch duplicate headers.
        let parts: Vec<&str> = req.splitn(2, "\r\n\r\n").collect();
        if parts.len() != 2 {
            return Err("Bad request format");
        }
        let header_part = parts[0];
        let body_part = parts[1];

        let mut lines = header_part.lines();
        let request_line = lines.next().ok_or("Missing request line")?;
        let mut req_line_parts = request_line.split_whitespace();
        let method = req_line_parts.next().ok_or("Missing method")?.to_string();
        let path = req_line_parts.next().ok_or("Missing path")?.to_string();

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((k, v)) = line.split_once(":") {
                headers.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        // Vulnerable section: unsafely update a mutable global variable.
        if let Some(cl_val) = headers.get("Content-Length") {
            if let Ok(cl) = cl_val.parse::<usize>() {
                unsafe {
                    // This unsafe block writes to a global variable without synchronization.
                    GLOBAL_CONTENT_LENGTH = cl;
                }
            }
        }

        // Further processing with unsafe pointer arithmetic.
        let len = body_part.len();
        if len > unsafe { GLOBAL_CONTENT_LENGTH } {
            unsafe {
                let ptr = body_part.as_ptr();
                // Use an unsafe slice extraction based solely on the global state.
                let slice = std::slice::from_raw_parts(ptr, unsafe { GLOBAL_CONTENT_LENGTH });
                let body = String::from_utf8_lossy(slice).to_string();
                return Ok(HttpRequest { method, path, headers, body: Some(body) });
            }
        }
        Ok(HttpRequest { method, path, headers, body: Some(body_part.to_string()) })
    }
}

// Public API for testing
pub fn parse_http(request: &str) -> Result<HttpRequest, &'static str> {
    ParserImpl::parse(request)
}

fn main() {
    // Simulate a request with duplicate Content-Length headers (a potential request smuggling scenario)
    let request = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n12345678901234567890";
    let arc_req = Arc::new(request.to_string());
    let mut handles = Vec::new();

    for _ in 0..4 {
        let req_clone = Arc::clone(&arc_req);
        let handle = thread::spawn(move || {
            match ParserImpl::parse(&req_clone) {
                Ok(parsed) => {
                    if let Some(body) = &parsed.body {
                        // Unsafe check: if body length does not match the (unsafely set) global value, panic.
                        unsafe {
                            if body.len() != GLOBAL_CONTENT_LENGTH {
                                panic!(
                                    "Body length mismatch: expected {}, got {}",
                                    GLOBAL_CONTENT_LENGTH,
                                    body.len()
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("Parsing failed: {}", e);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Finished processing in main.");
}