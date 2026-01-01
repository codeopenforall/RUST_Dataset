//////////////////////
// Vulnerable version
//////////////////////
use std::io::{Result, Write};
use std::sync::Arc;
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct HttpResponse {
    header: String,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { header: String::new() }
    }

    // Sets a header using untrusted input without sanitization.
    pub fn set_hdr(&mut self, name: &str, value: &str) {
        // CRLF injection vulnerability: value is directly embedded.
        self.header = format!("{}: {}\r\n", name, value);
        unsafe {
            // Simulate an unsafe memory usage with raw pointer operations.
            let c_string = CString::new(self.header.clone()).unwrap();
            let ptr: *const c_char = c_string.as_ptr();
            // Perform a meaningless unsafe read to mimic pointer misuse.
            std::ptr::read_volatile(ptr);
        }
    }

    pub fn get_hdr(&self) -> &str {
        &self.header
    }

    // Simulate sending the response (e.g., writing to a socket).
    pub fn send(&self) -> Result<()> {
        let mut out = std::io::stdout();
        out.write_all(self.header.as_bytes())?;
        Ok(())
    }
}

// Public function to generate a response header string from input.
// This is accessible by external testing code.
pub fn generate_response(input: &str) -> String {
    let mut resp = HttpResponse::new();
    resp.set_hdr("Location", input);
    resp.get_hdr().to_string()
}

// Function that runs the header construction in a background thread.
fn run_bg(input: String) {
    let shared_input = Arc::new(input);
    let shared_clone = Arc::clone(&shared_input);
    thread::spawn(move || {
        let mut resp = HttpResponse::new();
        resp.set_hdr("Location", &shared_clone);
        resp.send().unwrap();
    })
    .join()
    .unwrap();
}

fn main() {
    // Malicious input with CRLF injected header parts.
    let inj = "https://example.com\r\nInjected: malicious".to_string();
    run_bg(inj);
}