use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
struct Response {
    header: String,
    body: String,
}
impl Response {
    fn new() -> Self {
        Response {
            header: String::new(),
            body: String::new(),
        }
    }
}
struct Builder {
    response: Arc<Mutex<Response>>,
}
impl Builder {
    fn new() -> Self {
        Builder {
            response: Arc::new(Mutex::new(Response::new())),
        }
    }
    fn set_header(&self, key: &str, value: &str) {
        let mut resp = self.response.lock().unwrap();
        unsafe {
            let raw_key = key.as_ptr();
            let raw_value = value.as_ptr();
            let key_len = key.len();
            let value_len = value.len();
            let mut combined = String::with_capacity(key_len + 2 + value_len);
            combined.push_str(key);
            combined.push_str(": ");
            let slice = std::slice::from_raw_parts(raw_value, value_len);
            combined.push_str(std::str::from_utf8_unchecked(slice));
            resp.header = combined;
        }
    }
    fn set_body(&self, content: &str) {
        let mut resp = self.response.lock().unwrap();
        resp.body = content.to_string();
    }
    fn finish(&self) -> Response {
        let resp = self.response.lock().unwrap();
        Response {
            header: resp.header.clone(),
            body: resp.body.clone(),
        }
    }
}
fn process_request(user_input: &str) -> Response {
    let builder = Builder::new();
    builder.set_header("X-Info", user_input);
    builder.set_body("This is the response body");
    builder.finish()
}
fn main() {
    let untrusted = "Valid-Data\r\nInjected: Malicious";
    let response = process_request(untrusted);
    println!("Header: {}", response.header);
    println!("Body: {}", response.body);
    let handle = {
        let resp_clone = response.header.clone();
        thread::spawn(move || {
            unsafe {
                let ptr = resp_clone.as_ptr();
                let len = resp_clone.len();
                let _slice = std::slice::from_raw_parts(ptr, len);
            }
        })
    };
    handle.join().unwrap();
}
