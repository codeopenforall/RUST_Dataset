//////////////////////////////////////////////
// Vulnerable Code (Compilation Unit)
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

struct Renderer {
    data: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    // This function “renders” the untrusted input by directly copying
    // its bytes using an unsafe block. It does no escaping.
    fn render(&self, input: &str) -> String {
        let mut result = String::new();
        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            // Reinterpret raw bytes as &str, then use unescaped string.
            let s = std::str::from_utf8(slice).unwrap();
            result.push_str("Hello, ");
            // Vulnerability: untrusted input inserted without escaping.
            result.push_str(s);
        }
        result
    }

    // Simulate a concurrent rendering scenario.
    fn start(&self, input: String) -> String {
        let arc = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            let mut guard = arc.lock().unwrap();
            // Vulnerability: reflecting untrusted input into HTML without escaping.
            *guard = format!("<html><body>{}</body></html>", input);
            guard.clone()
        });
        handle.join().unwrap()
    }
}

// Public function to expose the core functionality for testing.
pub fn process_input(input: &str) -> String {
    let renderer = Renderer::new();
    renderer.start(renderer.render(input))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw = if args.len() > 1 { args[1].clone() } else { String::from("default") };
    let output = process_input(&raw);
    println!("{}", output);
}