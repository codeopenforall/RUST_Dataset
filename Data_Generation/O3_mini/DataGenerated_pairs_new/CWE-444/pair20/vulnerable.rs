///////////////////////
// Vulnerable Example
///////////////////////
use std::collections::HashMap;
use std::str;

struct HttpRequest {
    method: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    // Parses a raw HTTP request string using unsafe pointer arithmetic.
    // It relies solely on the user-supplied Content-Length header to determine the body size,
    // without verifying that the length is within the actual input bounds.
    // This unsafe copy can lead to an inconsistent interpretation of request boundaries.
    unsafe fn parse(input: &str) -> Self {
        // Find end of headers.
        let header_end = input.find("\r\n\r\n").unwrap_or(input.len());
        let header_str = &input[..header_end];
        let mut headers = HashMap::new();
        let mut method = String::new();
        
        // Parse the request line and headers.
        for (i, line) in header_str.lines().enumerate() {
            if i == 0 {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    method = parts[0].to_string();
                }
            } else {
                if let Some(pos) = line.find(":") {
                    let key = line[..pos].trim();
                    let val = line[pos+1..].trim();
                    headers.insert(key.to_string(), val.to_string());
                }
            }
        }
        
        // Vulnerable section: Compute the content length from the header without bounds checking.
        let content_length: usize = headers.get("Content-Length")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let body_offset = header_end + 4; // Skip the "\r\n\r\n"
        
        // UNSAFE: Copying data from the input buffer using the length provided by the header.
        let src = input.as_ptr().add(body_offset); // (Line 35)
        let mut buffer: Vec<u8> = Vec::with_capacity(content_length);
        buffer.set_len(content_length);         // (Line 37)
        std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), content_length); // (Line 38)
        let body = String::from_utf8_lossy(&buffer).into_owned();
        
        HttpRequest { method, headers, body }
    }
}

fn main() {
    // Example input simulating a request smuggling scenario.
    // The header declares a Content-Length that may include parts of a subsequent request.
    let request_str = "GET / HTTP/1.1\r\nContent-Length: 20\r\nHost: example.com\r\n\r\nGET /admin HTTP/1.1\r\n";
    // Unsafe parser invocation.
    let req = unsafe { HttpRequest::parse(request_str) };
    println!("Method: {}", req.method);
    println!("Body: {}", req.body);
}