/////////////////// Vulnerable Code ///////////////////
#![allow(unused)]
use std::collections::HashMap;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct HttpMsg {
    content_length: Option<usize>,
    is_chunked: bool,
    body: Vec<u8>,
}

impl HttpMsg {
    // Unsafe function that tries to process the body based on header values.
    // If both "Content-Length" and "Transfer-Encoding" are set,
    // it unsafely trusts the Content-Length header to slice the buffer,
    // even if the actual body is shorter.
    unsafe fn process(&self) -> Vec<u8> {
        if self.is_chunked && self.content_length.is_some() {
            let req_len = self.content_length.unwrap();
            let ptr_body = self.body.as_ptr();
            // Vulnerability: using content_length as slice size without verifying
            // it does not exceed the actual allocation.
            let slice = std::slice::from_raw_parts(ptr_body, req_len);
            slice.to_vec()
        } else {
            self.body.clone()
        }
    }
}

fn parse_req(request: &str) -> HttpMsg {
    let mut content_length = None;
    let mut is_chunked = false;
    let mut headers = HashMap::new();
    let mut lines = request.lines();
    let mut body = Vec::new();

    // Parse headers until an empty line is found.
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        if let Some((key, val)) = line.split_once(":") {
            headers.insert(key.trim().to_lowercase(), val.trim().to_string());
        }
    }

    if let Some(val) = headers.get("content-length") {
        if let Ok(num) = val.parse::<usize>() {
            content_length = Some(num);
        }
    }
    if let Some(te) = headers.get("transfer-encoding") {
        if te.to_lowercase().contains("chunked") {
            is_chunked = true;
        }
    }

    // The remainder is treated as the body.
    for line in lines {
        body.extend_from_slice(line.as_bytes());
    }

    HttpMsg { 
        content_length, 
        is_chunked, 
        body 
    }
}

fn main() {
    // This input has conflicting headers: Content-Length is 30 but the actual body is shorter.
    let input = "POST / HTTP/1.1\r\nContent-Length: 30\r\nTransfer-Encoding: chunked\r\n\r\nThis is the request body";
    let req = parse_req(input);
    let shared_req = Arc::new(req);
    let worker = {
        let req_clone = Arc::clone(&shared_req);
        thread::spawn(move || {
            unsafe {
                let processed = req_clone.process();
                // Print the processed output.
                println!("Processed output: {:?}", String::from_utf8_lossy(&processed));
            }
        })
    };

    worker.join().unwrap();
}