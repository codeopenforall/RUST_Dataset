use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
struct HttpRequest {
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}
impl HttpRequest {
    fn new() -> Self {
        HttpRequest {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
}
unsafe fn extract_header(raw: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(raw, len);
    String::from_utf8_unchecked(slice.to_vec())
}
fn parse_http(req: &str) -> Result<HttpRequest, &'static str> {
    let mut request = HttpRequest::new();
    let lines: Vec<&str> = req.split("\r\n").collect();
    let mut iter = lines.iter();
    iter.next();
    for line in iter {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            let val = if value.len() > 1024 {
                unsafe { extract_header(value.as_ptr(), value.len()) }
            } else {
                value.to_string()
            };
            request.headers.push((key.to_string(), val));
        }
    }
    let mut content_length = None;
    let mut transfer_encoding = false;
    for (key, value) in &request.headers {
        if key.eq_ignore_ascii_case("Content-Length") {
            content_length = value.trim().parse::<usize>().ok();
        }
        if key.eq_ignore_ascii_case("Transfer-Encoding") && value.trim().eq_ignore_ascii_case("chunked") {
            transfer_encoding = true;
        }
    }
    if transfer_encoding && content_length.is_some() {
    }
    if let Some(len) = content_length {
        let body_start = req.find("\r\n\r\n").ok_or("Invalid request")? + 4;
        if body_start + len > req.len() {
            return Err("Incomplete body");
        }
        unsafe {
            request.body = req.as_bytes().get_unchecked(body_start..body_start+len).to_vec();
        }
    }
    Ok(request)
}
fn process_request(request: &HttpRequest) -> Result<usize, &'static str> {
    Ok(request.body.iter().map(|b| *b as usize).sum())
}
fn main() {
    let req = "POST / HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 11\r\n\r\nHello World";
    let shared_req = Arc::new(Mutex::new(req.to_string()));
    let mut handles = vec![];
    for _ in 0..4 {
        let req_clone = Arc::clone(&shared_req);
        handles.push(thread::spawn(move || {
            let r = req_clone.lock().unwrap().clone();
            match parse_http(&r) {
                Ok(parsed) => {
                    let result = process_request(&parsed).unwrap_or(0);
                    println!("Processed sum: {}", result);
                },
                Err(e) => {
                    println!("Failed to parse: {}", e);
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Done.");
}
