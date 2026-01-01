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

fn parse_http(req: &str) -> Result<HttpRequest, &'static str> {
    let mut request = HttpRequest::new();
    let lines: Vec<&str> = req.split("\r\n").collect();
    let mut iter = lines.iter();
    // Skip the request line.
    iter.next();
    // Parse headers safely.
    for line in iter {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            request.headers.push((key.to_string(), value.to_string()));
        }
    }
    let mut content_length = None;
    let mut transfer_encoding = false;
    for (key, value) in &request.headers {
        if key.eq_ignore_ascii_case("Content-Length") {
            if let Ok(cl) = value.trim().parse::<usize>() {
                content_length = Some(cl);
            }
        }
        if key.eq_ignore_ascii_case("Transfer-Encoding") && value.trim().eq_ignore_ascii_case("chunked") {
            transfer_encoding = true;
        }
    }
    // Correct implementation: reject ambiguous requests containing both headers.
    if transfer_encoding && content_length.is_some() {
         return Err("Ambiguous request with both Transfer-Encoding and Content-Length headers");
    }
    // Safely extract the body.
    if let Some(len) = content_length {
        let body_start = req.find("\r\n\r\n").ok_or("Invalid request")? + 4;
        if body_start.checked_add(len).filter(|&end| end <= req.len()).is_none() {
            return Err("Incomplete body");
        }
        let body_slice = &req.as_bytes()[body_start..body_start+len];
        request.body = body_slice.to_vec();
    } else if transfer_encoding {
        // For a chunked transfer encoding, a full decoding routine would be required.
        let body_start = req.find("\r\n\r\n").ok_or("Invalid request")? + 4;
        request.body = req.as_bytes()[body_start..].to_vec();
    }
    Ok(request)
}

fn process_request(request: &HttpRequest) -> Result<usize, &'static str> {
    // Dummy processing: return the sum of the body bytes.
    Ok(request.body.iter().map(|b| *b as usize).sum())
}

fn main() {
    // The HTTP request contains ambiguous headers; this proper implementation rejects it.
    let req = "POST / HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 11\r\n\r\nHello World";
    let shared_req = Arc::new(Mutex::new(req.to_string()));
    let mut handles = vec![];
    for _ in 0..4 {
        let req_clone = Arc::clone(&shared_req);
        handles.push(thread::spawn(move || {
            let r = req_clone.lock().unwrap().clone();
            match parse_http(&r) {
                Ok(parsed) => {
                    match process_request(&parsed) {
                        Ok(result) => println!("Processed sum: {}", result),
                        Err(e) => println!("Processing error: {}", e),
                    }
                },
                Err(e) => println!("Parsing error: {}", e),
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Done.");
}