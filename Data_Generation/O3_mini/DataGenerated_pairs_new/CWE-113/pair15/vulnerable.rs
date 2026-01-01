///////////////////////////////
// Vulnerable Version - HTTP Header Builder with CRLF Injection Vulnerability
///////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Response {
    header: String,
}

impl Response {
    fn new(input: &str) -> Self {
        // Directly builds the header with unsanitized user input.
        let head = build_header(input);
        Response { header: head }
    }
}

// This function builds an HTTP header using an unsafe conversion.
// It does not sanitize CR/LF characters, which could allow HTTP response splitting.
fn build_header(user_data: &str) -> String {
    let prefix = "X-Custom: ";
    // Unsafe block used to directly convert untrusted bytes to a string.
    // This bypasses UTF8 verifications and fails to sanitize any control characters.
    unsafe {
        let unchecked = std::str::from_utf8_unchecked(user_data.as_bytes());
        format!("{}{}", prefix, unchecked)
    }
}

fn main() {
    // Using concurrency and a shared smart pointer to simulate real-world behavior.
    let shared_resp = Arc::new(Mutex::new(None::<Response>));
    let cloned_resp = Arc::clone(&shared_resp);
    let input_data = "user-input\r\nInjected: malicious-header"; // Malicious input with CRLF

    let handler = thread::spawn(move || {
        let resp = Response::new(input_data);
        let mut lock = cloned_resp.lock().unwrap();
        *lock = Some(resp);
    });

    handler.join().unwrap();

    // Display the resulting header.
    let lock = shared_resp.lock().unwrap();
    if let Some(resp) = &*lock {
        println!("Created header: {}", resp.header);
    }
}