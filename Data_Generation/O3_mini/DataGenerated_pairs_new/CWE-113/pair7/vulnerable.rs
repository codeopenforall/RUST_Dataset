use std::{ptr, sync::{Arc, Mutex}};
use std::thread;

pub struct HttpResponse {
    headers: String,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { headers: String::new() }
    }

    // Appends a header line using unsafe raw pointer manipulation.
    // This function does no sanitization of the header value.
    pub fn append_header(&mut self, key: &str, value: &str) {
        unsafe {
            // Construct the header line with CRLF termination.
            let line = format!("{}: {}\r\n", key, value);
            let bytes_len = line.len();
            let src = line.as_ptr();
            // Create a new vector and set its length without initializing its contents.
            let mut buf = Vec::with_capacity(bytes_len);
            buf.set_len(bytes_len);
            // Direct memory copy without bounds checking.
            ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), bytes_len);
            // Convert the raw bytes back to a string; assumes valid UTF-8.
            let header_line = String::from_utf8_unchecked(buf);
            self.headers.push_str(&header_line);
        }
    }

    pub fn construct(&self) -> String {
        let mut response = String::from("HTTP/1.1 200 OK\r\n");
        response.push_str(&self.headers);
        response.push_str("\r\n");
        response
    }
}

// Processes an untrusted header input and builds an HTTP response.
// Vulnerability: the untrusted input is not sanitized, allowing CRLF injection.
pub fn build_response(untrusted: &str) -> String {
    let mut resp = HttpResponse::new();
    // No check is made on 'untrusted', potentially allowing CRLF injection.
    resp.append_header("X-Custom-Header", untrusted);
    resp.construct()
}

// A worker function that simulates a concurrent header processing scenario.
fn process(untrusted: String, shared: Arc<Mutex<String>>) {
    let result = build_response(&untrusted);
    let mut guard = shared.lock().unwrap();
    *guard = result;
}

pub fn main() {
    let injected = "vulnerableValue\r\nInjected-Header: injectedValue";
    // Shared container to collect a response assembled from multiple threads.
    let shared_resp = Arc::new(Mutex::new(String::new()));
    let mut threads = vec![];

    for _ in 0..2 {
        let input = injected.to_string();
        let shared_clone = Arc::clone(&shared_resp);
        let t = thread::spawn(move || {
            process(input, shared_clone);
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
    let final_resp = shared_resp.lock().unwrap().clone();
    println!("{}", final_resp);
}