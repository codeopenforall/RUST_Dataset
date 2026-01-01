//////////////////////////////////////////////////////////////
// An HTTP response builder with unsafe header construction.
// Mimics real-world concurrency and unsafe memory usage.
//////////////////////////////////////////////////////////////

use std::thread;

struct HttpResp {
    headers: Vec<String>,
}

impl HttpResp {
    fn new() -> Self {
        HttpResp { headers: Vec::new() }
    }

    // Adds a header field without sanitizing the value.
    // UNSAFE: Direct conversion of untrusted input bypasses validation,
    // allowing CRLF sequences that enable HTTP response splitting (CWE-113).
    fn add_field(&mut self, key: &str, value: &str) {
        unsafe {
            let raw = value.as_bytes();
            // Vulnerability occurs here: no check for CR ('\r') and LF ('\n')
            let untrusted = std::str::from_utf8_unchecked(raw);
            let header_line = format!("{}: {}", key, untrusted);
            self.headers.push(header_line);
        }
    }

    fn get_fields(&self) -> &Vec<String> {
        &self.headers
    }
}

// Simulates processing that constructs an HTTP response from input.
// Processes the header in a separate thread to mimic concurrent behavior.
fn construct_response(input: &str) -> HttpResp {
    let mut resp = HttpResp::new();
    let input_clone = input.to_string();
    let handle = thread::spawn(move || {
        resp.add_field("X-Custom", &input_clone);
        resp
    });
    handle.join().unwrap()
}

fn main() {
    // Untrusted user input containing CRLF injection attempt.
    let user_input = "safe\r\nInjected: malicious";
    let response = construct_response(user_input);
    for field in response.get_fields() {
        println!("{}", field);
    }
}