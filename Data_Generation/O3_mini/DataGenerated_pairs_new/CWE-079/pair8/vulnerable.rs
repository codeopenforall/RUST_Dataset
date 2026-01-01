///////////////////////////////////////////////////////////////////////////////
// Vulnerable Code - CWE-079: Cross-Site Scripting (XSS)
// This implementation reflects untrusted input into an HTML template without
// any form of escaping. It uses an unsafe block to convert raw pointers and
// employs concurrency primitives to mimic a real-world asynchronous request.
///////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct PageTemplate {
    content: String,
}

impl PageTemplate {
    fn new() -> Self {
        PageTemplate {
            content: "<html><body>{content}</body></html>".to_string(),
        }
    }
    
    // Unsafe conversion of user-provided raw data into a &str.
    // The raw pointer is assumed to be valid UTF-8, bypassing any checks.
    unsafe fn render(&self, user_input: *const u8, len: usize) -> String {
        // Vulnerability: Untrusted input is directly used in the template without escaping.
        let slice = std::slice::from_raw_parts(user_input, len);
        let input_str = std::str::from_utf8_unchecked(slice);
        self.content.replace("{content}", input_str)
    }
}

fn process_request(input: String) -> String {
    // Emulate concurrent processing of user input.
    let template = Arc::new(PageTemplate::new());
    let user_data = Arc::new(input);
    let result = Arc::new(Mutex::new(String::new()));
    
    let template_clone = Arc::clone(&template);
    let user_clone = Arc::clone(&user_data);
    let result_clone = Arc::clone(&result);
    
    let handle = thread::spawn(move || {
        // Unsafe block to invoke the unsafe render method.
        let raw = user_clone.as_ptr();
        let len = user_clone.len();
        let page = unsafe { template_clone.render(raw, len) };
        let mut res = result_clone.lock().unwrap();
        *res = page;
    });
    
    handle.join().unwrap();
    Arc::try_unwrap(result).ok().unwrap().into_inner().unwrap()
}

fn main() {
    // Simulated untrusted user input, which contains a malicious script.
    let malicious = "<script>alert('XSS');</script>".to_string();
    let response = process_request(malicious);
    println!("{}", response);
}