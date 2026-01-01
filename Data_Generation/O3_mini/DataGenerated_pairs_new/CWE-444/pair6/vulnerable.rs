/////////////////////////////////////////////////////////////
// Vulnerability Example: Inconsistent HTTP Request Parsing //
/////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::str;

pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn parse_input(input: &str) -> Result<HTTPRequest, &'static str> {
    // Split the input into header and body sections.
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err("Malformed request: missing header/body separator");
    }
    let header_part = parts[0];
    let body_part = parts[1];

    let mut lines = header_part.lines();
    // Parse the request line.
    let request_line = lines.next().ok_or("Missing request line")?;
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    if req_parts.len() < 3 {
        return Err("Bad request line");
    }

    // Process headers using unsafe pointer arithmetic,
    // which bypasses bounds checks and proper duplicate header handling.
    let mut headers = HashMap::new();
    for line in lines {
        // Use an unsafe block to parse each header line.
        unsafe {
            let ptr = line.as_ptr();
            let mut pos = 0;
            // Locate the colon separator.
            while pos < line.len() && *ptr.add(pos) != b':' {
                pos += 1;
            }
            if pos >= line.len() {
                continue;
            }
            // Extract key and value without proper validation.
            let key = String::from_utf8_unchecked(Vec::from(&line.as_bytes()[0..pos]));
            // Skip the colon and any potential whitespace.
            let value = String::from_utf8_unchecked(Vec::from(&line.as_bytes()[pos+1..]));
            // Note: Duplicate headers (e.g., multiple Content-Length) are not properly handled.
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // Vulnerable behavior: Uses the (possibly ambiguous) Content-Length header.
    if let Some(cl) = headers.get("Content-Length") {
        if let Ok(n) = cl.parse::<usize>() {
            // Unsafe conversion: does not verify that 'n' is consistent with the actual body length.
            unsafe {
                let body_ptr = body_part.as_ptr();
                let body_slice = std::slice::from_raw_parts(body_ptr, n);
                let body_str = str::from_utf8_unchecked(body_slice);
                return Ok(HTTPRequest {
                    method: req_parts[0].to_string(),
                    path: req_parts[1].to_string(),
                    headers,
                    body: body_str.to_string(),
                });
            }
        }
    }

    Ok(HTTPRequest {
        method: req_parts[0].to_string(),
        path: req_parts[1].to_string(),
        headers,
        body: body_part.to_string(),
    })
}

fn main() {
    // This input has duplicate Content-Length headers:
    // The first header indicates 10 bytes and the second indicates 5 bytes.
    // Due to overwriting in the HashMap and unsafe processing, the second (incorrect) value is used.
    let request = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 5\r\n\r\nHelloWorld";
    match parse_input(request) {
        Ok(parsed) => {
            println!("Method: {}", parsed.method);
            println!("Path: {}", parsed.path);
            println!("Headers: {:?}", parsed.headers);
            // The body is unsafely truncated to 5 bytes ("Hello") due to using the wrong header value.
            println!("Body: {}", parsed.body);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}