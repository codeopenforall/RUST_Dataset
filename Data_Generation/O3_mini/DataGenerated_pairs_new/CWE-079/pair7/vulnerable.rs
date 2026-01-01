//
// Note: This code intentionally demonstrates unsafe rendering of HTML content without proper escaping.
// It uses unsafe blocks and concurrency to mimic patterns seen in real-world systems vulnerable to XSS (CWE-079).
//
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    template: String,
}

impl Renderer {
    fn new() -> Self {
        Renderer { template: "<html><body>{}</body></html>".to_string() }
    }
    
    // Render method that directly reflects untrusted input into the HTML template.
    // It uses an unsafe block to convert the input's bytes to a string without validation.
    fn generate(&self, input: &str) -> String {
        unsafe {
            let bytes = input.as_bytes();
            // Bypass runtime checks and assume the input is always valid.
            let untrusted = std::str::from_utf8_unchecked(bytes);
            // Insert untrusted content directly into the template.
            self.template.replace("{}", untrusted)
        }
    }
}

fn main() {
    // Using smart pointers and concurrency to simulate real-world usage.
    let renderer = Arc::new(Mutex::new(Renderer::new()));
    let renderer_clone = Arc::clone(&renderer);
    let handle = thread::spawn(move || {
        // Potentially malicious user input with an embedded script.
        let input = "<script>alert('XSS');</script>";
        let html = renderer_clone.lock().unwrap().generate(input);
        println!("{}", html);
    });
    handle.join().unwrap();
}