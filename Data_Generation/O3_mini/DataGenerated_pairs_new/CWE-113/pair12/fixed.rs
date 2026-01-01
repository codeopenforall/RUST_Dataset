//////////////////////
// Fixed Version
//////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct HttpResponse {
    header: String,
}

impl HttpResponse {
    // Constructs an HTTP response with proper sanitization of header values.
    pub fn new(input: &str) -> HttpResponse {
        // Sanitize the input by removing carriage return and newline characters
        let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();

        HttpResponse {
            header: format!("Location: {}", sanitized),
        }
    }

    pub fn send(&self) -> String {
        self.header.clone()
    }
}

fn main() {
    // The same untrusted input, but it will be sanitized in this version.
    let input = "http://example.com\r\nSet-Cookie: sessionid=bad";
    let response = Arc::new(Mutex::new(HttpResponse::new(input)));

    let mut threads = vec![];
    for _ in 0..4 {
        let clone_response = Arc::clone(&response);
        threads.push(thread::spawn(move || {
            let resp = clone_response.lock().unwrap();
            let header = resp.send();
            println!("{}", header);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}