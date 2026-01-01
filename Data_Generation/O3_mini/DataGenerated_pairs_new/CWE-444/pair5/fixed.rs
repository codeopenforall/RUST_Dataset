/////////////////// Corrected Code ///////////////////
#![allow(unused)]
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct HttpMsg {
    content_length: Option<usize>,
    is_chunked: bool,
    body: Vec<u8>,
}

impl HttpMsg {
    // Safely process the body by ignoring the Content-Length header
    // when Transfer-Encoding indicates chunked encoding. Also verifies that
    // if Content-Length is used, it does not exceed the actual body length.
    unsafe fn process(&self) -> Vec<u8> {
        if self.is_chunked {
            // If chunked, rely on the actual body.
            self.body.clone()
        } else if let Some(req_len) = self.content_length {
            // Check if the requested length exceeds the actual allocation.
            let actual_len = self.body.len();
            let safe_len = if req_len > actual_len { actual_len } else { req_len };
            let ptr_body = self.body.as_ptr();
            let slice = std::slice::from_raw_parts(ptr_body, safe_len);
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

    // Parse headers until an empty line is encountered.
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

    // The remainder is the body.
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
    // When conflicting headers exist, the implementation now prioritizes
    // the Transfer-Encoding header and uses the actual body for processing.
    let input = "POST / HTTP/1.1\r\nContent-Length: 30\r\nTransfer-Encoding: chunked\r\n\r\nThis is the request body";
    let req = parse_req(input);
    let shared_req = Arc::new(req);
    let worker = {
        let req_clone = Arc::clone(&shared_req);
        thread::spawn(move || {
            unsafe {
                let processed = req_clone.process();
                println!("Processed output: {:?}", String::from_utf8_lossy(&processed));
            }
        })
    };

    worker.join().unwrap();
}