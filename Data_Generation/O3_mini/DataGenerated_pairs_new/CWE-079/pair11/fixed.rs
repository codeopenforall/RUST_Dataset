//////////////////////////////////////////////
// A Rust program simulating a web renderer 
// with proper HTML escaping to prevent XSS.
//////////////////////////////////////////////

use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

struct Renderer {
    // Internal HTML template with a placeholder for dynamic content.
    template: String,
}

impl Renderer {
    fn new() -> Self {
        Renderer { template: "<html><body>{content}</body></html>".to_string() }
    }

    // Renders the page by safely inserting escaped user input into the template.
    fn render(&self, input: &str) -> String {
        let safe_input = escape_html(input);
        self.template.replace("{content}", &safe_input)
    }
}

// Escapes characters that have special meaning in HTML.
fn escape_html(input: &str) -> String {
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
}

// Entry function that processes the input concurrently.
fn handle_input(input: String) -> String {
    let renderer = Renderer::new();
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