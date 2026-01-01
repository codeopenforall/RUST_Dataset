//////////////////////////////////////////////////////
// Vulnerable implementation simulating an HTTP parser
// with inconsistent header interpretation using unsafe
// operations and concurrent thread execution.
//////////////////////////////////////////////////////
use std::collections::HashMap;
use std::str;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    // Unsafe parser that uses raw pointer conversions.
    // It naively overrides duplicate headers.
    unsafe fn parse_http(input: &[u8]) -> Result<Self, &'static str> {
        // Convert the input slice via pointer, imitating low‐level access.
        let ptr = input.as_ptr();
        let len = input.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let mut headers = HashMap::new();
        let mut index = 0;
        // Parse header lines until an empty line is found.
        while index < slice.len() {
            let mut line_end = index;
            while line_end < slice.len() && slice[line_end] != b'\n' {
                line_end += 1;
            }
            // Empty line detection (a header line with 0 or 1 char before newline)
            if line_end - index <= 1 {
                index = line_end + 1;
                break;
            }
            let line = &slice[index..line_end];
            if let Some(colon_pos) = line.iter().position(|&b| b == b':') {
                let key = String::from_utf8_lossy(&line[..colon_pos]).trim().to_string();
                let value = String::from_utf8_lossy(&line[colon_pos + 1..]).trim().to_string();
                // Inconsistent behavior: if a header is repeated (e.g., Content-Length),
                // the parser overwrites the previous value, leading to a different interpretation.
                headers.insert(key, value);
            }
            index = line_end + 1;
        }
        // Retrieve Content-Length header unsafely.
        let cl_val = headers.get("Content-Length").ok_or("Missing Content-Length")?;
        let body_len: usize = cl_val.parse().map_err(|_| "Invalid Content-Length")?;
        if index + body_len > slice.len() {
            return Err("Body length exceeds available data");
        }
        // Unsafe extraction of the body without additional validation.
        let body = slice[index..index + body_len].to_vec();
        Ok(Request { headers, body })
    }
}

fn main() {
    // Simulated raw HTTP request with duplicate Content-Length headers.
    let raw = b"POST / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n0123456789ABCDEFGHIJKLMNOPQRST";
    
    // Spawn a thread to simulate concurrent processing.
    let handle = thread::spawn(move || {
        unsafe {
            match Request::parse_http(raw) {
                Ok(req) => {
                    // This prints the body that was extracted using the last Content-Length header.
                    println!("Parsed body: {:?}", str::from_utf8(&req.body).unwrap());
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    });
    handle.join().unwrap();
}