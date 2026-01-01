//////////////////////////////////////////////
// Vulnerable Code: Insecure HTTP request parser 
// (Simulated CWE-444: Inconsistent Interpretation of HTTP Requests)
//////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    // Unsafe parsing using raw pointer dereference.
    pub unsafe fn parse(raw: &str) -> Self {
        // Simulate low-level conversion with unsafe pointer arithmetic.
        let ptr = raw.as_ptr();
        let len = raw.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let s = String::from_utf8_lossy(slice).to_string();

        let mut lines = s.lines();
        let request_line = lines.next().unwrap_or("");
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let uri = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        // Insecure extraction of body: relies solely on Content-Length.
        let cl = headers
            .get("Content-Length")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        // Calculate body start using total length of input string.
        let s_bytes = s.as_bytes();
        let body_start = s.len().saturating_sub(cl);
        // Vulnerability: if there is a conflicting Transfer-Encoding header,
        // extra bytes from a smuggled second request might be appended to the body.
        let body = s_bytes[body_start..].to_vec();

        Request {
            method,
            uri,
            version,
            headers,
            body,
        }
    }

    // Insecure processing: ignores header conflicts.
    // If both "Content-Length" and "Transfer-Encoding" exist, it does not reject.
    pub fn process(&self) -> Result<(), &'static str> {
        // Vulnerable: even if both headers exist, the request is accepted.
        if self.headers.contains_key("Content-Length")
            && self.headers.contains_key("Transfer-Encoding")
        {
            // Instead of rejecting ambiguous requests, it processes them.
            Ok(())
        } else {
            Ok(())
        }
    }
}

// A public API that executes the request processing.
pub fn execute_request(raw: &str) -> Result<(), &'static str> {
    let req = unsafe { Request::parse(raw) };
    // Spawn a thread to simulate concurrent processing.
    let req_arc = Arc::new(req);
    let req_clone = Arc::clone(&req_arc);
    let handler = thread::spawn(move || req_clone.process());
    handler.join().unwrap()
}

fn main() {
    // Malicious request with ambiguous headers.
    let raw_request = "POST /upload HTTP/1.1\r\n\
                       Host: example.com\r\n\
                       Content-Length: 11\r\n\
                       Transfer-Encoding: chunked\r\n\r\n\
                       Hello World\r\n\
                       GET /malicious HTTP/1.1\r\n\
                       Host: evil.com\r\n\r\n";
    // In the vulnerable code, ambiguous requests are accepted.
    match execute_request(raw_request) {
        Ok(_) => println!("Request processed insecurely."),
        Err(e) => println!("Error processing request: {}", e),
    }
}