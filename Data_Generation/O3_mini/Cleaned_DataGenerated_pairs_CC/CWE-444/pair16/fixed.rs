use std::str;
use std::io::{self, Write};
struct Context {
    data: Vec<u8>,
}
impl Context {
    fn new(input: &str) -> Self {
        Self { data: input.as_bytes().to_vec() }
    }
}
fn process_request(request: &str) -> Result<u64, &'static str> {
    let mut content_length_val: Option<u64> = None;
    let request_str = match str::from_utf8(request.as_bytes()) {
        Ok(r) => r,
        Err(_) => return Err("Invalid UTF-8 in request"),
    };
    for line in request_str.split("\r\n") {
        if line.starts_with("Content-Length:") {
            let header_value = line["Content-Length:".len()..].trim();
            if let Ok(value) = header_value.parse::<u64>() {
                if content_length_val.is_some() {
                    return Err("Ambiguous Content-Length header");
                }
                content_length_val = Some(value);
            } else {
                return Err("Invalid Content-Length value");
            }
        }
    }
    content_length_val.ok_or("Missing Content-Length header")
}
fn main() {
    let malicious_request = "POST / HTTP/1.1\r\n\
                             Host: example.com\r\n\
                             Content-Length: 5\r\n\
                             Content-Length: 10\r\n\
                             \r\n\
                             helloWorld";
    match process_request(malicious_request) {
        Ok(len) => {
            println!("Parsed Content-Length: {}", len);
        },
        Err(e) => {
            eprintln!("Error processing request: {}", e);
        }
    }
}
