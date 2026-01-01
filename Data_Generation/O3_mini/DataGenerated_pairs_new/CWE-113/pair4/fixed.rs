use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct Responder {
    header: String,
}

impl Responder {
    fn new(input: &str) -> Self {
        // Rebuilding the header with sanitized input to prevent CRLF injection.
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nX-Custom: {}\r\n\r\n",
            input
        );
        Responder { header }
    }

    fn get_header(&self) -> &str {
        self.header.as_str()
    }
}

// Sanitization function that removes CR and LF characters.
fn sanitize(input: &str) -> String {
    input.replace('\r', "").replace('\n', "")
}

fn build_response(input: &str) -> Responder {
    let cleaned = sanitize(input);
    unsafe {
        let c_input = CString::new(cleaned.clone()).unwrap();
        let raw_ptr = c_input.as_ptr();
        let safe_str = CStr::from_ptr(raw_ptr).to_str().unwrap();
        Responder::new(safe_str)
    }
}

fn main() {
    // Read external input; now any CRLF sequences will be removed.
    let arg = env::args().nth(1).unwrap_or_else(|| String::from("Normal-Value"));
    let response = build_response(&arg);
    let shared = Arc::new(Mutex::new(response));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let guard = shared_clone.lock().unwrap();
        println!("{}", guard.get_header());
    });
    handle.join().unwrap();
}