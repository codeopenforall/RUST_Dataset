use std::{ptr, sync::{Arc, Mutex}};
use std::thread;

pub struct HttpResponse {
    headers: String,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse { headers: String::new() }
    }

    // Safely appends a header line. The header value is sanitized to remove CR and LF characters.
    pub fn append_header(&mut self, key: &str, value: &str) {
        // Sanitize the value by removing CR and LF characters.
        let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        unsafe {
            let line = format!("{}: {}\r\n", key, safe_value);
            let bytes_len = line.len();
            let src = line.as_ptr();
            let mut buf = Vec::with_capacity(bytes_len);
            buf.set_len(bytes_len);
            ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), bytes_len);
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

// Processes the header input and builds an HTTP response.
// The header value is sanitized to mitigate CRLF injection.
pub fn build_response(untrusted: &str) -> String {
    let mut resp = HttpResponse::new();
    resp.append_header("X-Custom-Header", untrusted);
    resp.construct()
}

fn process(untrusted: String, shared: Arc<Mutex<String>>) {
    let result = build_response(&untrusted);
    let mut guard = shared.lock().unwrap();
    *guard = result;
}

pub fn main() {
    let injected = "vulnerableValue\r\nInjected-Header: injectedValue";
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