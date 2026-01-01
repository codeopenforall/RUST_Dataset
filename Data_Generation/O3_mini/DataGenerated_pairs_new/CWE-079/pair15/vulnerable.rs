///////////////////////////////////////////////////////////////
// Vulnerable implementation (unsafe XSS reflection without escaping)
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::str;

struct Renderer {
    tpl: String,
}

impl Renderer {
    fn new(template: &str) -> Self {
        Renderer {
            tpl: template.to_owned(),
        }
    }
    
    // Renders HTML by substituting user-supplied data directly into the template.
    // Uses an unsafe block to bypass certain checks and reflects the untrusted input.
    fn render(&self, input: &str) -> String {
        let mut output = self.tpl.clone();
        unsafe {
            // Dangerous: directly reconstructs an &str from the raw bytes of input.
            // No sanitization is performed, leading to potential XSS.
            let bytes = input.as_bytes();
            let untrusted = str::from_utf8_unchecked(bytes);
            output = output.replace("{user_data}", untrusted);
        }
        output
    }
}

fn concurrent_process(renderer: Arc<Mutex<Renderer>>, input: String) -> String {
    let handle = thread::spawn(move || {
        // Concurrency with shared state; locking the renderer.
        let renderer = renderer.lock().unwrap();
        renderer.render(&input)
    });
    handle.join().unwrap()
}

fn main() {
    // HTML template with a placeholder for user data.
    let tpl = "<html><body>Hello, {user_data}!</body></html>";
    let renderer = Arc::new(Mutex::new(Renderer::new(tpl)));
    
    // Untrusted input that can inject malicious JavaScript.
    let user_input = "<script>alert('XSS');</script>".to_string();
    let result = concurrent_process(renderer, user_input);
    
    println!("Output: {}", result);
}