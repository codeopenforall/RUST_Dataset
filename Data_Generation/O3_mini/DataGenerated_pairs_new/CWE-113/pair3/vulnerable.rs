///////////////////////////////////////////////////////////////
// Vulnerability: HTTP Response Splitting (CWE-113)
// This code builds HTTP response headers using an untrusted input
// for a header value. It uses an unsafe block that converts a raw
// pointer to a string slice without verifying the absence of CR
// and LF characters. This may allow an attacker to inject CRLF
// sequences to split headers (HTTP response splitting).
///////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::str;

struct ResponseBuilder {
    headers: Vec<String>,
}

impl ResponseBuilder {
    fn new() -> Self {
        ResponseBuilder { headers: Vec::new() }
    }

    // Adds a header without sanitizing the value.
    fn add_header(&mut self, key: &str, value: &str) {
        // Vulnerable: no validation on `value`, unsafe conversion used.
        unsafe {
            let raw_value = value.as_ptr();
            let len = value.len();
            let value_str = str::from_utf8_unchecked(std::slice::from_raw_parts(raw_value, len));
            self.headers.push(format!("{}: {}", key, value_str));
        }
    }

    fn build(&self) -> String {
        let mut result = String::new();
        for header in &self.headers {
            result.push_str(header);
            result.push_str("\r\n");
        }
        // Terminate header section with an extra CRLF.
        result.push_str("\r\n");
        result
    }
}

// Public function that processes the header using the input string.
// This function is called by main as well as by our external test.
pub fn process(input: &str) -> String {
    let builder = Arc::new(Mutex::new(ResponseBuilder::new()));
    let safe_input = input.to_owned();
    let clone_builder = builder.clone();

    let handle = thread::spawn(move || {
        let mut rb = clone_builder.lock().unwrap();
        rb.add_header("X-Custom", &safe_input);
    });
    handle.join().unwrap();
    let rb = builder.lock().unwrap();
    rb.build()
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let response = process(&arg);
    println!("{}", response);
}