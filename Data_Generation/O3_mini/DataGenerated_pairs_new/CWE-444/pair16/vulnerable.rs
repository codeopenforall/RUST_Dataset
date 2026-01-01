//////////////////////////////
// Insecure HTTP Request Processor (vulnerable)
//////////////////////////////
use std::str;
use std::io::{self, Write};

struct Context {
    // Simulate a buffer for HTTP request
    data: Vec<u8>,
}

impl Context {
    fn new(input: &str) -> Self {
        Self { data: input.as_bytes().to_vec() }
    }
}

fn process_request(request: &str) -> Result<u64, &'static str> {
    // This function unsafely parses the HTTP headers to extract the Content-Length.
    // It does not reject duplicates and uses an unsafe block to bypass UTF-8 checks.
    let mut content_length_val: Option<u64> = None;
    
    unsafe { // [Line 17] Unsafe conversion and header parsing (vulnerability)
        // Directly convert the raw bytes to &str without proper validations.
        let ptr = request.as_ptr();
        let len = request.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        let request_str = str::from_utf8_unchecked(raw_slice);
        
        for line in request_str.split("\r\n") {
            if line.starts_with("Content-Length:") {
                // Vulnerability: Does not check if multiple headers are present.
                let header_value = line["Content-Length:".len()..].trim();
                if let Ok(value) = header_value.parse::<u64>() {
                    // Overwrite previously parsed value if duplicate header exists.
                    content_length_val = Some(value); // [Line 24] Insecure duplicate header handling.
                } else {
                    return Err("Invalid Content-Length value");
                }
            }
        }
    }
    
    content_length_val.ok_or("Missing Content-Length header")
}

fn main() {
    // Simulate an incoming HTTP request with ambiguous duplicate headers.
    // In a production proxy, different components might interpret this differently.
    let malicious_request = "POST / HTTP/1.1\r\n\
                             Host: example.com\r\n\
                             Content-Length: 5\r\n\
                             Content-Length: 10\r\n\
                             \r\n\
                             helloWorld";
    
    match process_request(malicious_request) {
        Ok(len) => {
            // Potentially smuggle the request if one component uses 5 and another 10.
            println!("Parsed Content-Length: {}", len);
        },
        Err(e) => {
            eprintln!("Error processing request: {}", e);
        }
    }
}