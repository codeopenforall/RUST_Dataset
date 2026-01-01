///////////////////////////////
// Vulnerable Code Start
///////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    buffer: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Renderer {
            buffer: Arc::new(Mutex::new(String::new())),
        }
    }

    // This function processes user input by unsafely copying it
    // into a shared buffer and then embedding it directly into HTML.
    // NOTE: It does not perform any escaping.
    fn render(&self, input: &str) -> String {
        let cloned_buffer = Arc::clone(&self.buffer);
        let input_owned = input.to_owned();
        let handle = thread::spawn(move || {
            // Unsafe block used to simulate pointer manipulation without bounds checks.
            unsafe {
                let ptr = input_owned.as_ptr();
                let len = input_owned.len();
                let slice = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
                let mut lock = cloned_buffer.lock().unwrap();
                lock.push_str(slice);
            }
        });
        handle.join().unwrap();
        // Vulnerability: User-controlled input is directly embedded into HTML.
        let current = self.buffer.lock().unwrap();
        format!("<html><body><div>{}</div></body></html>", *current)
    }
}

fn main() {
    // For demonstration: reading input from command line argument.
    let input = env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let engine = Renderer::new();
    let result = engine.render(&input);
    println!("{}", result);
}
///////////////////////////////
// Vulnerable Code End
///////////////////////////////