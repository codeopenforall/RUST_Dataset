//////////////////// Vulnerable Code ////////////////////

use std::env;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Responder {
    headers: Arc<Mutex<Vec<u8>>>,
}

impl Responder {
    fn new() -> Self {
        Responder {
            headers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // This function appends a header without sanitizing the value.
    fn append_header(&self, key: &str, value: &str) {
        // Build a header line "Key: Value\r\n"
        let header_line = format!("{}: {}\r\n", key, value);
        let bytes = header_line.as_bytes();
        let len = bytes.len();
        let mut buffer = Vec::with_capacity(len);

        // Unsafe block that mimics low-level write using raw pointers.
        unsafe {
            buffer.set_len(len);
            // Directly copy the bytes into the uninitialized memory without bounds check.
            // This operation poses a risk if header_line comes from untrusted sources.
            ptr::copy_nonoverlapping(bytes.as_ptr(), buffer.as_mut_ptr(), len);
        }

        // Simulate additional work in a separate thread
        let headers_cloned = Arc::clone(&self.headers);
        thread::spawn(move || {
            let mut h = headers_cloned.lock().unwrap();
            h.extend_from_slice(&buffer);
        })
        .join()
        .expect("Thread panicked");
    }

    // Combine headers into a single HTTP response string.
    fn build_response(&self) -> String {
        let h = self.headers.lock().unwrap();
        String::from_utf8_lossy(&h).to_string()
    }
}

fn main() {
    // Assume the untrusted value is passed as the first CLI argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <header value>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];

    let responder = Responder::new();
    // Vulnerability: using unsanitized input for header value.
    responder.append_header("X-Inject", input);

    let response = responder.build_response();
    println!("{}", response);
}