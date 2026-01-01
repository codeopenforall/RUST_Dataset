////////////////////////////////////////////////////////////////
// Corrected code example with strict parsing to avoid request smuggling.
// The parser now safely iterates over the headers and explicitly rejects
// requests with ambiguous Transfer-Encoding and Content-Length headers.
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
    let mut lines_iter = raw.split("\r\n");

    // Parse the request line.
    let request_line = lines_iter.next()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    req.method = parts[0].to_string();
    req.uri = parts[1].to_string();
    
    // Safely parse headers.
    let mut header_map = HashMap::new();
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }
        if let Some(pos) = line.find(":") {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            header_map.insert(key, value);
        }
    }
    req.headers = header_map;

    // Enforce strict header rules: reject if both ambiguous headers exist.
    if req.headers.contains_key("Content-Length") && req.headers.contains_key("Transfer-Encoding") {
        return None; // Reject ambiguous requests.
    }
    
    // Safely collect the remainder of the message as the body.
    req.body = lines_iter.collect::<Vec<&str>>().join("\r\n");
    Some(req)
}

fn run() {
    // Use a benign input that does not contain ambiguous headers.
    let benign = "POST / HTTP/1.1\r\nHost: secure\r\nContent-Length: 13\r\n\r\nGET /admin HTTP/1.1\r\n";
    
    // Simulate concurrent access using a shared state.
    let shared_state = Arc::new(Mutex::new(String::from("normal")));
    let state_clone = shared_state.clone();
    thread::spawn(move || {
        let mut data = state_clone.lock().unwrap();
        *data = String::from("modified");
    }).join().unwrap();
    
    if let Some(req) = parse_http(benign) {
        println!("Parsed method: {}", req.method);
        println!("Parsed uri: {}", req.uri);
        println!("Parsed headers: {:?}", req.headers);
        println!("Parsed body: {}", req.body);
        // The benign input should not be misinterpreted to include a smuggled request.
        assert!(!req.body.contains("GET /admin"), "Unexpected request smuggling detected!");
    } else {
        panic!("Parsing failed on benign input");
    }
}

fn main() {
    run();
}