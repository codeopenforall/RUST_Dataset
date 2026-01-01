use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Responder {
    header: Vec<u8>,
}

impl Responder {
    fn new() -> Self {
        Responder { header: vec![0; 1024] }
    }

    // Build a response header by unsafely inserting user data without sanitizing CR and LF.
    fn create_response(&mut self, input: &str) -> String {
        let base = b"HTTP/1.1 302 Found\r\nLocation: ";
        // Copy fixed header part into buffer.
        unsafe {
            ptr::copy_nonoverlapping(base.as_ptr(), self.header.as_mut_ptr(), base.len());
        }
        // UNSAFE: Insert the raw user-provided string without filtering CR/LF.
        unsafe {
            let dest = self.header.as_mut_ptr().add(base.len());
            let src = input.as_ptr();
            let len = input.len();
            ptr::copy_nonoverlapping(src, dest, len);
        }
        // Append end-of-header marker.
        let end = b"\r\n\r\n";
        unsafe {
            let dest = self.header.as_mut_ptr().add(base.len() + input.len());
            ptr::copy_nonoverlapping(end.as_ptr(), dest, end.len());
        }
        let total_len = base.len() + input.len() + end.len();
        // Assume the result is always valid UTF-8.
        unsafe { String::from_utf8_unchecked(self.header[..total_len].to_vec()) }
    }
}

fn simulate(input: String) -> String {
    let response = Arc::new(Mutex::new(Responder::new()));
    let resp_clone = Arc::clone(&response);
    let handle = thread::spawn(move || {
        let mut responder = resp_clone.lock().unwrap();
        responder.create_response(&input)
    });

    handle.join().unwrap()
}

fn main() {
    // Malicious input with CRLF sequences enabling header injection.
    let malicious = "http://example.com\r\nInjected-Header: injected_value".to_string();
    let res = simulate(malicious);
    println!("{}", res);
}