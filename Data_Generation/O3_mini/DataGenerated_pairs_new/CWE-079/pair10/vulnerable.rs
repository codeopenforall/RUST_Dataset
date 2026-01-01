//////////////////////////////////////////////
// Realistic XSS reflection in HTML generation //
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    template: String,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { template: "<html><body>{content}</body></html>".to_string() }
    }

    pub fn render(&self, user_input: &str) -> String {
        // Clone template into mutable string.
        let mut result = self.template.clone();
        unsafe {
            // Unsafe block simulating low-level memory manipulation.
            // The placeholder is located and then directly replaced with untrusted input.
            let placeholder = "{content}";
            let _ptr = result.as_mut_ptr(); // Raw pointer (unused beyond simulation)
            let mut idx = 0;
            while idx < result.len() {
                if result.as_bytes()[idx..].starts_with(placeholder.as_bytes()) {
                    break;
                }
                idx += 1;
            }
            if idx < result.len() {
                let before = &result[..idx];
                let after = &result[idx + placeholder.len()..];
                // Direct insertion of untrusted input results in Cross-Site Scripting possibility.
                result = format!("{}{}{}", before, user_input, after);
            }
        }
        result
    }
}

fn main() {
    // Shared Renderer instance used concurrently.
    let renderer = Arc::new(Mutex::new(Renderer::new()));
    let untrusted_input = "<script>alert('XSS');</script>";
    let mut handles = Vec::new();

    for _ in 0..5 {
        let render_clone = Arc::clone(&renderer);
        let input_clone = untrusted_input.to_string();
        let handle = thread::spawn(move || {
            let guard = render_clone.lock().unwrap();
            let output = guard.render(&input_clone);
            output
        });
        handles.push(handle);
    }

    for handle in handles {
        let output = handle.join().unwrap();
        println!("{}", output);
    }
}