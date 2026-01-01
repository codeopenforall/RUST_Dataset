////////////////////////////////////////////////////////////////////////
// Secure HTML template rendering code.
// This version mitigates Cross-Site Scripting (CWE-079) by properly escaping
// untrusted user input. It retains a similar structure with concurrency and
// unsafe constructs (now removed from the core rendering logic) to mimic real-world
// systems while fixing the vulnerability.
//
use std::sync::{Arc, Mutex};
use std::thread;

// Simple HTML escaping function for a subset of critical characters.
fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '&' => "&amp;".to_owned(),
            '"' => "&quot;".to_owned(),
            '\'' => "&#x27;".to_owned(),
            _ => c.to_string()
        }
    }).collect()
}

struct Renderer {
    data: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Renderer {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    // Securely renders an HTML template after escaping any untrusted input.
    fn render(&self, input: &str) -> String {
        let safe_input = escape_html(input);
        format!("<html><body>Hello, {}!</body></html>", safe_input)
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
            let instance = Renderer { data: clone };
            let payload = text.to_string();
            thread::spawn(move || {
                // Render the template with proper escaping.
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