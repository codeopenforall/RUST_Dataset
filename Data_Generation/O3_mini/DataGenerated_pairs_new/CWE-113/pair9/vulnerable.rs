//////////////////////////////////////////////
// A real-world-like HTTP header responder
// (CWE-113: Improper Neutralization of CRLF Sequences)
// This code demonstrates an unsafe concatenation operation
// where user-provided data is inserted directly into HTTP headers
// without sanitization. An unsafe block is used to simulate low-level
// pointer manipulation common in legacy code, and the lack of input
// filtering leads to HTTP response splitting when CR/LF are present.
//////////////////////////////////////////////
use std::io::{self, Write};

struct HttpResponder;

impl HttpResponder {
    // Constructs an HTTP redirect header using user-provided input.
    // This version directly uses the input value inside an unsafe block
    // to mimic low-level pointer operations without sanitizing the input.
    pub fn reply(&self, location: &str) -> String {
        unsafe {
            // Convert the input string to a raw pointer and reconstruct it.
            // No check or cleaning is done.
            let raw = location.as_ptr();
            let len = location.len();
            let slice = std::slice::from_raw_parts(raw, len);
            let location_str = std::str::from_utf8_unchecked(slice);
            format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", location_str)
        }
    }
}

fn main() {
    // Malicious input contains CRLF characters for header injection.
    let input = "http://example.com\r\nSet-Cookie: malicious=1";
    let responder = HttpResponder;
    let response = responder.reply(input);
    println!("{}", response);
}