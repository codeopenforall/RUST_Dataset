//////////////////////////////////////////////
// A secure version of the HTTP header responder
// that properly sanitizes user input to prevent
// HTTP response splitting (CWE-113).
// The code still employs an unsafe block for pointer manipulation,
// but first filters out any carriage return or line feed characters.
//////////////////////////////////////////////
use std::io::{self, Write};

struct HttpResponder;

impl HttpResponder {
    // Constructs an HTTP redirect header using sanitized input.
    // This version filters out CR and LF characters before using the input
    // inside an unsafe block, thereby neutralizing any injection attempts.
    pub fn reply(&self, location: &str) -> String {
        unsafe {
            // Remove CR and LF characters from the input.
            let sanitized: String = location.chars().filter(|&c| c != '\r' && c != '\n').collect();
            let raw = sanitized.as_ptr();
            let len = sanitized.len();
            let slice = std::slice::from_raw_parts(raw, len);
            let location_str = std::str::from_utf8_unchecked(slice);
            format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", location_str)
        }
    }
}

fn main() {
    // For demonstration, using a benign URL.
    // Even if a malicious input is provided, the reply() method will sanitize it.
    let input = "http://example.com";
    let responder = HttpResponder;
    let response = responder.reply(input);
    println!("{}", response);
}