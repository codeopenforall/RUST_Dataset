////////////////////////////////////////////////////////////////////////
// Vulnerability demonstration code for HTML template rendering.
// This code unsafely reflects user input into HTML without any escaping,
// enabling Cross-Site Scripting (CWE-079) attacks. It also demonstrates
// unsafe Rust usage and concurrent execution.
//
// Note: Do not use this code in production!
//
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    data: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Renderer {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    // Renders an HTML template by directly embedding user-provided text.
    // Uses an unsafe block to unsafely convert the input bytes into &str.
    fn render(&self, input: &str) -> String {
        // Unsafe block: assumes input is valid UTF-8 without sanitization.
        // This is the core vulnerability: unsanitized inclusion of untrusted input.
        unsafe {
            let raw_input: &str = std::str::from_utf8_unchecked(input.as_bytes());
            format!("<html><body>Hello, {}!</body></html>", raw_input)
        }
    }
}

fn main() {
    let renderer = Renderer::new();
    // Simulated user inputs: a normal string and a malicious string.
    let inputs = vec!["Alice", "<script>alert('XSS')</script>"];
    // Spawn threads to simulate concurrent rendering.
    let threads: Vec<_> = inputs
        .into_iter()
        .map(|text| {
            let clone = renderer.data.clone();
            // Cloning the renderer instance for each thread.
            let instance = Renderer { data: clone };
            let payload = text.to_string();
            thread::spawn(move || {
                // Render the template unsafely.
                instance.render(&payload)
            })
        })
        .collect();

    // Collect the results from threads.
    for t in threads {
        let result = t.join().expect("Thread panicked");
        // Update the shared buffer (simulate concurrent modification).
        let mut locked = renderer.data.lock().unwrap();
        *locked = result.clone();
        println!("{}", result);
    }
}