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
    pub unsafe fn parse(raw: &str) -> Self {
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
        let cl = headers
            .get("Content-Length")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        let s_bytes = s.as_bytes();
        let body_start = s.len().saturating_sub(cl);
        let body = s_bytes[body_start..].to_vec();
        Request {
            method,
            uri,
            version,
            headers,
            body,
        }
    }
    pub fn process(&self) -> Result<(), &'static str> {
        if self.headers.contains_key("Content-Length")
            && self.headers.contains_key("Transfer-Encoding")
        {
            Err("Ambiguous header: both Content-Length and Transfer-Encoding are set")
        } else {
            Ok(())
        }
    }
}
pub fn execute_request(raw: &str) -> Result<(), &'static str> {
    let req = unsafe { Request::parse(raw) };
    let req_arc = Arc::new(req);
    let req_clone = Arc::clone(&req_arc);
    let handler = thread::spawn(move || req_clone.process());
    handler.join().unwrap()
}
fn main() {
    let raw_request = "POST /upload HTTP/1.1\r\n\
                       Host: example.com\r\n\
                       Content-Length: 11\r\n\
                       Transfer-Encoding: chunked\r\n\r\n\
                       Hello World\r\n\
                       GET /malicious HTTP/1.1\r\n\
                       Host: evil.com\r\n\r\n";
    match execute_request(raw_request) {
        Ok(_) => println!("Request processed insecurely."),
        Err(e) => println!("Error processing request: {}", e),
    }
}
