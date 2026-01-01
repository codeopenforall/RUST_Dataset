//////////////////////////////////////////////////////////////
// Vulnerable Code Sample - CWE-079 (XSS Reflection)
// This code unsafely interpolates untrusted user input into an HTML template
// using an unsafe block to convert bytes into a string without verification.
//////////////////////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

fn render(input: &str) -> String {
    // UNSAFE: Converts the input bytes to &str without proper validation.
    let unsafe_str = unsafe { std::str::from_utf8_unchecked(input.as_bytes()) };
    // Directly interpolates the untrusted input into the HTML template.
    format!("<html><body><h1>Welcome {}</h1></body></html>", unsafe_str)
}

fn main() {
    // Read input from command-line arguments.
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 { args[1].clone() } else { String::new() };

    // Shared pointer with Mutex to simulate concurrent context.
    let shared = Arc::new(Mutex::new(user_input));
    let shared_clone = Arc::clone(&shared);

    // Spawn a thread to render the page.
    let handle = thread::spawn(move || {
        let input = shared_clone.lock().unwrap();
        let page = render(&input);
        // Output the generated HTML page.
        println!("{}", page);
    });

    handle.join().unwrap();
}