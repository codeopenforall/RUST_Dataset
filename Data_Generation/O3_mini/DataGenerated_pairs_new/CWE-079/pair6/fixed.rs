////////////////////////////
// Fixed code snippet
////////////////////////////

use std::thread;

fn escape_html(input: &str) -> String {
    // Escapes critical HTML characters to prevent injection.
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('\"', "&quot;")
         .replace('\'', "&#x27;")
}

struct Renderer;

impl Renderer {
    // This safe method escapes untrusted input before inserting it into the HTML template.
    pub fn build_html(&self, input: &str) -> String {
        let safe_input = escape_html(input);
        let mut html = String::from("<html><body>User: ");
        html.push_str(&safe_input);
        html.push_str("</body></html>");
        html
    }
}

// A helper function that spawns a thread to simulate concurrent rendering.
fn process_input(input: &str) -> String {
    let renderer = Renderer;
    let inp = input.to_owned();
    let handle = thread::spawn(move || renderer.build_html(&inp));
    handle.join().unwrap()
}

fn main() {
    // Simulated malicious input remains the same.
    let malicious = "<script>alert('XSS');</script>";
    // The rendering function now escapes the input, preventing XSS.
    let output = process_input(malicious);
    println!("{}", output);
}