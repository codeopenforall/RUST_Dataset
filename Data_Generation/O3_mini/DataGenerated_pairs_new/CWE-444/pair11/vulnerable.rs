////////////////////////////////////////////////////////////////
// Vulnerable code example demonstrating improper HTTP request parsing
// leading to inconsistent interpretation between headers.
// This code uses unsafe pointer arithmetic and unchecked slice accesses.
////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct HttpRequest {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    fn new() -> Self {
        HttpRequest {
            method: String::new(),
            uri: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

fn parse_http(raw: &str) -> Option<HttpRequest> {
    let mut req = HttpRequest::new();
    let mut lines = raw.split("\r\n");
    if let Some(request_line) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        req.method = parts[0].to_string();
        req.uri = parts[1].to_string();
    } else {
        return None;
    }
    let mut header_map = HashMap::new();
    let mut reached_body = false;

    // Using unsafe pointer arithmetic to find boundaries in the raw request.
    unsafe {
        let raw_ptr = raw.as_ptr();
        let raw_len = raw.len();
        let mut index = 0;
        // Scan for the double-CRLF that separates headers and body.
        while index < raw_len - 3 {
            let slice = std::slice::from_raw_parts(raw_ptr.add(index), 4);
            if slice == b"\r\n\r\n" {
                reached_body = true;
                break;
            }
            index += 1;
        }
        // Vulnerability: reading header part without proper boundary checks.
        let header_part = std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, index));
        for line in header_part.lines().skip(1) {
            if let Some(pos) = line.find(":") {
                let key = line[..pos].trim().to_string();
                let value = line[pos+1..].trim().to_string();
                header_map.insert(key, value);
            }
        }
        req.headers = header_map;
        if reached_body {
            // Vulnerability: if both "Content-Length" and "Transfer-Encoding" exist,
            // the code chooses Content-Length without validating the inconsistency.
            if req.headers.contains_key("Content-Length") {
                let cl: usize = req.headers.get("Content-Length").unwrap().parse().unwrap_or(0);
                let body_start = index + 4; // assumes exact start of body after header block
                if body_start + cl <= raw_len {
                    let body_ptr = raw_ptr.add(body_start);
                    req.body = String::from_utf8_lossy(std::slice::from_raw_parts(body_ptr, cl)).to_string();
                }
            } else {
                // Default: use the rest of the input as the body.
                let body_slice = std::slice::from_raw_parts(raw_ptr.add(index + 4), raw_len - index - 4);
                req.body = String::from_utf8_lossy(body_slice).to_string();
            }
        }
    }
    Some(req)
}

fn run() {
    // Crafted input with ambiguous headers that can trigger request smuggling.
    let crafted = "POST / HTTP/1.1\r\nHost: vulnerable\r\nContent-Length: 13\r\nTransfer-Encoding: chunked\r\n\r\nGET /admin HTTP/1.1\r\n";
    
    // Simulate concurrent modification by a background thread.
    let shared_state = Arc::new(Mutex::new(String::from("normal")));
    let state_clone = shared_state.clone();
    thread::spawn(move || {
        let mut data = state_clone.lock().unwrap();
        *data = String::from("modified");
    }).join().unwrap();
    
    if let Some(req) = parse_http(crafted) {
        println!("Parsed method: {}", req.method);
        println!("Parsed uri: {}", req.uri);
        println!("Parsed headers: {:?}", req.headers);
        println!("Parsed body: {}", req.body);
        // Vulnerability effect: if the body contains a smuggled request line,
        // the program panics indicating detection of request smuggling.
        if req.body.contains("GET /admin") {
            panic!("Request smuggling detected!");
        }
    } else {
        println!("Failed to parse request");
    }
}

fn main() {
    run();
}