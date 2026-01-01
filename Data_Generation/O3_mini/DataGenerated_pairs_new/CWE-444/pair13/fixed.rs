//////////////////////////////////////////////////////
// Corrected HTTP parser implementation that validates
// for duplicate Content-Length headers and thus avoids
// inconsistent interpretation.
//////////////////////////////////////////////////////
use std::collections::HashMap;
use std::str;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    // Safe parser that rejects duplicate Content-Length headers.
    fn parse_http(input: &[u8]) -> Result<Self, &'static str> {
        let mut headers = HashMap::new();
        let mut index = 0;
        // Parse header lines until encountering an empty line.
        while index < input.len() {
            let mut line_end = index;
            while line_end < input.len() && input[line_end] != b'\n' {
                line_end += 1;
            }
            if line_end - index <= 1 {
                index = line_end + 1;
                break;
            }
            let line = &input[index..line_end];
            if let Some(colon_pos) = line.iter().position(|&b| b == b':') {
                let key = String::from_utf8_lossy(&line[..colon_pos]).trim().to_string();
                let value = String::from_utf8_lossy(&line[colon_pos + 1..]).trim().to_string();
                // Reject duplicate Content-Length header to avoid inconsistent interpretations.
                if key.eq_ignore_ascii_case("Content-Length") && headers.contains_key("Content-Length") {
                    return Err("Duplicate Content-Length header found");
                }
                headers.insert(key, value);
            }
            index = line_end + 1;
        }
        let cl_val = headers.get("Content-Length").ok_or("Missing Content-Length")?;
        let body_len: usize = cl_val.parse().map_err(|_| "Invalid Content-Length")?;
        if index + body_len > input.len() {
            return Err("Body length exceeds available data");
        }
        let body = input[index..index + body_len].to_vec();
        Ok(Request { headers, body })
    }
}

fn main() {
    // Simulated raw HTTP request with duplicate Content-Length headers.
    let raw = b"POST / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n0123456789ABCDEFGHIJKLMNOPQRST";
    
    // Spawn a thread to simulate concurrent processing.
    let handle = thread::spawn(move || {
        match Request::parse_http(raw) {
            Ok(req) => {
                println!("Parsed body: {:?}", str::from_utf8(&req.body).unwrap());
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });
    handle.join().unwrap();
}