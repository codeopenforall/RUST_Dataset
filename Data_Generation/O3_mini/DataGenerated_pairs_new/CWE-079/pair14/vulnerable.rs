///////////////////////////////////////////////////////////
// Realistic example of a web rendering module with unsafe
// unvalidated user input insertion (CWE-079: XSS vulnerability).
///////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    template: String,
}

impl Renderer {
    // Renders an HTML template by directly inserting untrusted input
    // without proper escaping.
    fn render(&self, user_input: &str) -> String {
        let mut output = self.template.clone();
        unsafe {
            // Unsafe block mimicking real-world misuse of raw pointers
            // to process untrusted input.
            let ptr = user_input.as_ptr();
            let len = user_input.len();
            let content = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
            // Directly substitutes the marker with unescaped user input.
            output = output.replace("{input}", content);
        }
        output
    }
}

fn run() {
    let template = "<html><body>Welcome, {input}!</body></html>".to_string();
    let renderer = Renderer { template };
    let shared = Arc::new(Mutex::new(renderer));

    // Spawn several threads to simulate concurrent request processing.
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let lock = Arc::clone(&shared);
            thread::spawn(move || {
                let user_input = "<script>alert('xss');</script>"; // malicious payload
                let guard = lock.lock().unwrap();
                let result = guard.render(user_input);
                // Output is printed directly, including unsanitized user input.
                println!("{}", result);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    run();
}