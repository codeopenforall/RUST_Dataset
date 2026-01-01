//////////////////////////////////////////////
// Corrected Code (Compilation Unit)
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

fn html_escape(input: &str) -> String {
    // Escape characters that could lead to XSS.
    input.chars().fold(String::new(), |mut acc, ch| {
        match ch {
            '<' => acc.push_str("&lt;"),
            '>' => acc.push_str("&gt;"),
            '&' => acc.push_str("&amp;"),
            '"' => acc.push_str("&quot;"),
            '\'' => acc.push_str("&#x27;"),
            _ => acc.push(ch),
        }
        acc
    })
}

struct Processor {
    data: Arc<Mutex<String>>,
}

impl Processor {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    // Process the untrusted input using an unsafe block as before,
    // but then escape it before further use.
    fn process(&self, input: &str) -> String {
        let mut result = String::new();
        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let s = std::str::from_utf8(slice).unwrap();
            // Escape the untrusted input to mitigate XSS.
            let safe = html_escape(s);
            result.push_str("Hello, ");
            result.push_str(&safe);
        }
        result
    }

    // Concurrent execution remains, but uses the already escaped input.
    fn run(&self, processed: String) -> String {
        let arc = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            let mut guard = arc.lock().unwrap();
            // Safe rendering: inserted input has been escaped.
            *guard = format!("<html><body>{}</body></html>", processed);
            guard.clone()
        });
        handle.join().unwrap()
    }
}

// Public function to expose the core functionality for testing.
pub fn process_input(input: &str) -> String {
    let processor = Processor::new();
    processor.run(processor.process(input))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw = if args.len() > 1 { args[1].clone() } else { String::from("default") };
    let output = process_input(&raw);
    println!("{}", output);
}