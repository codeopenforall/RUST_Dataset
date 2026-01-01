use std::sync::{Arc, Mutex};
use std::thread;
struct HttpRequest {
    method: String,
    uri: String,
    version: String,
    headers: Vec<(String, String)>,
    body: String,
}
impl HttpRequest {
    unsafe fn parse_body(raw: *const u8, len: usize) -> String {
        let slice = std::slice::from_raw_parts(raw, len);
        String::from_utf8_lossy(slice).into_owned()
    }
    pub fn parse(raw: &str) -> Result<Self, String> {
        let mut lines = raw.lines();
        let start_line = lines.next().ok_or("Empty request")?;
        let parts: Vec<&str> = start_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid request line".into());
        }
        let method = parts[0].to_string();
        let uri = parts[1].to_string();
        let version = parts[2].to_string();
        let mut headers: Vec<(String, String)> = Vec::new();
        let mut transfer_encoding = false;
        let mut content_length: Option<usize> = None;
        for line in &mut lines {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let header_parts: Vec<&str> = line.splitn(2, ":").collect();
            if header_parts.len() != 2 {
                continue;
            }
            let key = header_parts[0].trim().to_ascii_lowercase();
            let value = header_parts[1].trim().to_string();
            if key == "transfer-encoding" && value.to_ascii_lowercase() == "chunked" {
                transfer_encoding = true;
            }
            if key == "content-length" {
                if let Ok(v) = value.parse::<usize>() {
                    content_length = Some(v);
                }
            }
            headers.push((key, value));
        }
        let body_str: &str = raw.split("\r\n\r\n").nth(1).unwrap_or("");
        let body: String;
        if transfer_encoding && content_length.is_some() {
            unsafe {
                let ptr = body_str.as_ptr().offset(1); 
                let len = content_length.unwrap();
                body = HttpRequest::parse_body(ptr, len);
            }
        } else if transfer_encoding {
            body = body_str.replace("\r\n", "");
        } else if let Some(len) = content_length {
            body = body_str.chars().take(len).collect();
        } else {
            body = body_str.to_string();
        }
        Ok(HttpRequest { method, uri, version, headers, body })
    }
}
fn process_request(raw: &str) -> Result<String, String> {
    let shared_req = Arc::new(Mutex::new(None));
    let raw_copy = raw.to_string();
    let shared_req_thread = Arc::clone(&shared_req);
    let handle = thread::spawn(move || {
        let req = HttpRequest::parse(&raw_copy);
        let mut guard = shared_req_thread.lock().unwrap();
        *guard = req.ok();
    });
    handle.join().unwrap();
    let guard = shared_req.lock().unwrap();
    if let Some(ref req) = *guard {
        Ok(req.body.clone())
    } else {
        Err("Failed to parse request".into())
    }
}
fn main() {
    let input = "POST /test HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 5\r\n\r\nHelloWorld";
    match process_request(input) {
        Ok(body) => println!("Parsed body: {}", body),
        Err(e) => println!("Error: {}", e),
    }
}
