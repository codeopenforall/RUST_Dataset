///////////////////////////////
// Fixed Code Start
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

    // HTML escaping function: escapes &, <, >, ", and ' characters.
    fn escape(input: &str) -> String {
        input.chars().fold(String::new(), |mut acc, c| {
            match c {
                '&' => acc.push_str("&amp;"),
                '<' => acc.push_str("&lt;"),
                '>' => acc.push_str("&gt;"),
                '"' => acc.push_str("&quot;"),
                '\'' => acc.push_str("&#x27;"),
                _ => acc.push(c),
            }
            acc
        })
    }

    // This function processes user input by copying it into a shared buffer
    // and then embedding a sanitized version into HTML.
    fn render(&self, input: &str) -> String {
        let cloned_buffer = Arc::clone(&self.buffer);
        let input_owned = input.to_owned();
        let handle = thread::spawn(move || {
            // Even though an unsafe block is still used to mimic complex processing,
            // the input is properly handled afterwards.
            unsafe {
                let ptr = input_owned.as_ptr();
                let len = input_owned.len();
                let slice = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
                let mut lock = cloned_buffer.lock().unwrap();
                lock.push_str(slice);
            }
        });
        handle.join().unwrap();
        let current = self.buffer.lock().unwrap();
        // Apply escape on the untrusted input before embedding in HTML.
        let safe_content = Self::escape(&current);
        format!("<html><body><div>{}</div></body></html>", safe_content)
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
// Fixed Code End
///////////////////////////////