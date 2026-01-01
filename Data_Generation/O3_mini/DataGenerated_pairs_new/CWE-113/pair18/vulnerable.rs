///////////////////////////////
// Vulnerable Code Snippet
///////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct HeaderBuilder {
    header: Mutex<String>,
}

impl HeaderBuilder {
    fn new() -> Self {
        HeaderBuilder {
            header: Mutex::new(String::new()),
        }
    }

    // This unsafe function concatenates untrusted input into an HTTP header without sanitization.
    unsafe fn append_header(&self, input: &str) {
        let base = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-Custom: ";
        // CRLF injection vulnerability: no sanitization of input
        let combined = String::from(base) + input + "\r\n\r\n";
        *self.header.lock().unwrap() = combined;
    }

    fn get_header(&self) -> String {
        self.header.lock().unwrap().clone()
    }
}

pub fn build_response(input: &str) -> String {
    let builder = Arc::new(HeaderBuilder::new());
    let builder_clone = Arc::clone(&builder);
    let input_owned = input.to_owned();
    let handle = thread::spawn(move || {
        unsafe {
            builder_clone.append_header(&input_owned);
        }
    });
    handle.join().unwrap();
    builder.get_header()
}

fn main() {
    // Simulated untrusted input containing CRLF sequences
    let untrusted = "GoodHeader\r\nInjected: evil-value";
    let response = build_response(untrusted);
    println!("{}", response);
}