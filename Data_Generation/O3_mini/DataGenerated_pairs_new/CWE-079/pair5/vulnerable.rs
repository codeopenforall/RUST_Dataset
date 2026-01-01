/*
This program demonstrates a simple HTML rendering based on user‐supplied input.
It uses unsafe code to convert an input string into a raw slice without sanitization.
This leads to an XSS vulnerability (CWE-079) by reflecting untrusted input directly into HTML.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    content: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Renderer {
            content: Arc::new(Mutex::new(String::new())),
        }
    }

    fn build_page(&self, input: &str) -> String {
        let mut page = String::from("<html><body>");
        // Unsafe block: We manually convert the input into a byte slice and then into a String.
        // No escaping is performed, leaving the page vulnerable to XSS if input contains HTML/JS.
        unsafe {
            let raw_ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            // Directly inserting the unvalidated input into the HTML template:
            let unescaped = String::from_utf8_lossy(slice);
            page.push_str(&format!("<div>{}</div>", unescaped));
        }
        page.push_str("</body></html>");
        page
    }

    fn update_content(&self, text: String) {
        let shared = Arc::clone(&self.content);
        let handle = thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            *guard = text;
        });
        handle.join().unwrap();
    }
}

fn main() {
    // Simulate a web endpoint that reflects the user-supplied input.
    let args: Vec<String> = std::env::args().collect();
    let user_input = if args.len() > 1 { &args[1] } else { "default" };
    let renderer = Renderer::new();
    let page = renderer.build_page(user_input);
    println!("{}", page);
}