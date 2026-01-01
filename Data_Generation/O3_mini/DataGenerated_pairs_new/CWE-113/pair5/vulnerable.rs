//////////////////////////////////////////////
// Vulnerable HTTP header creation sample
//////////////////////////////////////////////
#![allow(unused_imports)]
use std::sync::Arc;
use std::thread;

struct Server {
    header: String,
}

impl Server {
    // Create a new instance from untrusted input without sanitizing CR or LF.
    fn new(input: &str) -> Self {
        // POTENTIAL FLAW: Untrusted input is injected directly into HTTP headers.
        let header = format!("HTTP/1.1 200 OK\r\nCustom-Header: {}\r\n", input);
        Server { header }
    }

    fn send(&self) -> String {
        // Simulate sending header using an unsafe block and raw pointer access.
        let header_ptr = self.header.as_ptr();
        let header_len = self.header.len();
        unsafe {
            let raw_slice = std::slice::from_raw_parts(header_ptr, header_len);
            String::from_utf8_lossy(raw_slice).to_string()
        }
    }
}

fn main() {
    // Simulated untrusted input that includes CRLF injection.
    let input = "malicious\r\nInjected: bad";
    let server = Arc::new(Server::new(input));
    let server_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        server_clone.send()
    });
    let response = handle.join().unwrap();
    println!("{}", response);
}