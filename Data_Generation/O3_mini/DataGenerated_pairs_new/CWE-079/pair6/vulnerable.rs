////////////////////////////
// Vulnerable code snippet
////////////////////////////

use std::thread;

struct Renderer;

impl Renderer {
    // This method uses an unsafe block to convert raw bytes into a string.
    // It directly interpolates the untrusted input into an HTML template without escaping.
    pub unsafe fn build_html(&self, input: &str) -> String {
        let mut html = String::from("<html><body>User: ");
        // UNSAFE: Converting raw pointer data back to &str without validation.
        let ptr = input.as_ptr();
        let len = input.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        let untrusted = std::str::from_utf8_unchecked(raw_slice);
        html.push_str(untrusted);
        html.push_str("</body></html>");
        html
    }
}

// A helper function that spawns a thread to simulate concurrent rendering.
fn process_input(input: &str) -> String {
    let renderer = Renderer;
    let inp = input.to_owned();
    let handle = thread::spawn(move || unsafe { renderer.build_html(&inp) });
    handle.join().unwrap()
}

fn main() {
    // Simulated malicious user input that includes HTML/JS.
    let malicious = "<script>alert('XSS');</script>";
    // The unsafe rendering reflects untrusted input without any sanitization.
    let output = process_input(malicious);
    println!("{}", output);
}