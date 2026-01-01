///////////////////////////////////////////////////////////
// Revised example that securely handles untrusted input
// by properly escaping HTML special characters to mitigate
// XSS (CWE-079) risks.
///////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    template: String,
}

impl Renderer {
    // Securely renders the HTML template by escaping untrusted input.
    fn render(&self, user_input: &str) -> String {
        let escaped = html_escape(user_input);
        let mut output = self.template.clone();
        output = output.replace("{input}", &escaped);
        output
    }
}

// Escapes critical HTML characters in the input string.
fn html_escape(input: &str) -> String {
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('\"', "&quot;")
         .replace('\'', "&#x27;")
}

fn run() {
    let template = "<html><body>Welcome, {input}!</body></html>".to_string();
    let renderer = Renderer { template };
    let shared = Arc::new(Mutex::new(renderer));

    // Spawn threads to simulate concurrent request processing.
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let lock = Arc::clone(&shared);
            thread::spawn(move || {
                let user_input = "<script>alert('xss');</script>"; // malicious payload
                let guard = lock.lock().unwrap();
                let result = guard.render(user_input);
                // The output now contains escaped characters so the script cannot run.
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