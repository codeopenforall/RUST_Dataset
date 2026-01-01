//////////////////////////////////////////////
// A Rust program simulating a web renderer 
// that unsafely reflects untrusted input into HTML
//////////////////////////////////////////////

use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

struct PageRenderer {
    // Internal HTML template with a placeholder for dynamic content.
    template: String,
}

impl PageRenderer {
    fn new() -> Self {
        PageRenderer { template: "<html><body>{content}</body></html>".to_string() }
    }

    // Renders the page by directly inserting untrusted input into the template.
    // This function unsafely manipulates the String's internal buffer.
    fn render(&self, input: &str) -> String {
        // Directly substitute the placeholder with input.
        let mut result = self.template.replace("{content}", input);
        unsafe {
            // UNSAFE: Directly access and modify the internal byte vector. 
            // This simulates a realistic misuse of unsafe blocks. The untrusted
            // input is appended again without any validation or bounds checks.
            let bytes = result.as_mut_vec();
            for &b in input.as_bytes() {
                bytes.push(b);
            }
        }
        result
    }
}

// Entry function that processes the input concurrently.
fn handle_input(input: String) -> String {
    let renderer = PageRenderer::new();
    let shared_output = Arc::new(Mutex::new(String::new()));
    let shared_clone = Arc::clone(&shared_output);

    // Spawn a thread to render the page concurrently.
    let handle = thread::spawn(move || {
        let rendered = renderer.render(&input);
        let mut guard = shared_clone.lock().unwrap();
        *guard = rendered;
    });
    handle.join().unwrap();
    Arc::try_unwrap(shared_output).unwrap().into_inner().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Accept untrusted input from a command-line argument.
    let user_input = if args.len() > 1 { args[1].clone() } else { "default".to_string() };
    let output = handle_input(user_input);
    println!("{}", output);
}