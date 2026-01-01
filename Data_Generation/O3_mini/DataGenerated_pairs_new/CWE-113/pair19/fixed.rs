//////////////////////////////////////////////////////////////
// Secure HTTP response builder with proper header sanitization.
// Corrects the issue by filtering out CR and LF characters.
//////////////////////////////////////////////////////////////

use std::thread;

struct HttpResp {
    headers: Vec<String>,
}

impl HttpResp {
    fn new() -> Self {
        HttpResp { headers: Vec::new() }
    }

    // Safely adds a header field by sanitizing the value input.
    // The sanitization removes CR ('\r') and LF ('\n') characters,
    // preventing HTTP response splitting.
    fn add_field(&mut self, key: &str, value: &str) {
        let sanitized: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let header_line = format!("{}: {}", key, sanitized);
        self.headers.push(header_line);
    }

    fn get_fields(&self) -> &Vec<String> {
        &self.headers
    }
}

// Simulates processing that constructs an HTTP response from input.
// Like its counterpart, it uses threading to mimic concurrent processing.
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
    // Untrusted user input attempting CRLF injection.
    // The sanitization ensures that any CR or LF characters are removed.
    let user_input = "safe\r\nInjected: malicious";
    let response = construct_response(user_input);
    for field in response.get_fields() {
        println!("{}", field);
    }
}