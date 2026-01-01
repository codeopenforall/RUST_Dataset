////////////////////////////////////////////////////////////
// Vulnerable Code - CWE-444: Inconsistent Interpretation
//
// This version implements a rudimentary HTTP proxy parser that 
// unsafely computes the location and length of a request body.
// It uses an unsafe block to transform raw pointer arithmetic 
// to get the body slice. When a request contains both a 
// "Content-Length" header and a "Transfer-Encoding: chunked" 
// header, it unconditionally uses the Content-Length value to 
// determine the body length, ignoring the fact that the backend 
// and proxy might parse the body differently. This misinterpretation
// can lead to request smuggling vulnerabilities.
////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::thread;

struct Request {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

// Unsafe conversion from pointer/length to string without bounds check.
unsafe fn unsafe_str<'a>(ptr: *const u8, len: usize) -> &'a str {
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
}

// Parses a raw HTTP request string into a Request struct.
// In the presence of both "Content-Length" and "Transfer-Encoding",
// it uses the numeric Content-Length value, ignoring proper chunked parsing.
fn parse_request(input: &str) -> Request {
    // Split headers and body based on CRLF CRLF.
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let header_str = parts.get(0).unwrap_or(&"");
    let mut headers = HashMap::new();
    let mut lines = header_str.lines();
    let request_line = lines.next().unwrap_or("");
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = req_parts.get(0).unwrap_or(&"").to_string();
    let uri = req_parts.get(1).unwrap_or(&"").to_string();

    for line in lines {
        if let Some((k, v)) = line.split_once(":") {
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    // Compute the starting pointer for the body unsafely.
    let header_ptr = header_str.as_ptr();
    let header_len = header_str.len();
    // Assume separator "\r\n\r\n" is exactly 4 bytes.
    let body_start = header_ptr as usize + header_len + 4;
    let total_ptr = input.as_ptr();
    let total_len = input.len();
    let body_len = if let Some(cl) = headers.get("Content-Length") {
        // Directly using the provided length without validation.
        cl.parse::<usize>().unwrap_or(0)
    } else {
        total_len - (body_start - total_ptr as usize)
    };

    // Unsafe block: Create a slice for the body without further validation.
    let body = unsafe {
        let body_slice = std::slice::from_raw_parts(body_start as *const u8, body_len);
        body_slice.to_vec()
    };

    Request { method, uri, headers, body }
}

// This function processes the HTTP input data.
fn process_input(input: &str) -> Vec<u8> {
    let req = parse_request(input);
    // Vulnerability: Even if Transfer-Encoding is specified,
    // the code uses Content-Length to cut the body.
    req.body
}

fn main() {
    // Simulated HTTP request with conflicting headers.
    let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n0\r\n\r\nExtra";
    let t = thread::spawn(move || {
        let result = process_input(input);
        println!("Processed body: {:?}", String::from_utf8_lossy(&result));
    });
    t.join().unwrap();
}