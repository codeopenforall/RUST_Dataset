//////////////////////
// Vulnerable Version
//////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct HttpResponse {
    header: String,
}

impl HttpResponse {
    // Constructs an HTTP response using untrusted input directly in header.
    // Unsafe blocks simulate low-level memory manipulation seen in real-world code.
    pub fn new(input: &str) -> HttpResponse {
        // Emulate unsafe memory operations (for realism, though not strictly needed)
        unsafe {
            let mut tmp = input.as_bytes().to_vec();
            let ptr = tmp.as_mut_ptr();
            // Note: This unsafe manipulation does nothing beyond mimic risky behavior.
            // It intentionally avoids any sanitization of CRLF characters.
            *ptr = *ptr;
        }
        HttpResponse {
            // Vulnerability: The untrusted input is directly injected into the header,
            // allowing CRLF injection and HTTP response splitting.
            header: format!("Location: {}", input),
        }
    }

    pub fn send(&self) -> String {
        // In a realistic server, this string would be sent as a header value.
        self.header.clone()
    }
}

fn main() {
    // Simulated untrusted input containing CRLF that can split HTTP response headers
    let input = "http://example.com\r\nSet-Cookie: sessionid=bad";
    let response = Arc::new(Mutex::new(HttpResponse::new(input)));

    let mut threads = vec![];
    // Simulate concurrent access in a multi-threaded server context
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