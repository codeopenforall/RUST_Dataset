//////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust program implements a simple HTML page renderer that directly embeds user-supplied
// content into an HTML template without any escaping. The code uses an unsafe block to reconstruct
// the string from a raw pointer and also demonstrates a threaded execution using smart pointers.
// Because no HTML escaping is performed, a malicious user input (containing tags like <script>)
// can be reflected into the generated HTML page, leading to Cross-Site Scripting vulnerabilities.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

pub struct Page {
    template: String,
}

impl Page {
    pub fn new(template: &str) -> Self {
        Page {
            template: template.to_string(),
        }
    }

    // This method assembles the HTML page by unsafely reading the user input bytes,
    // without any escaping or sanitization.
    pub fn assemble(&self, user_input: &str) -> String {
        let ptr = user_input.as_ptr();
        let len = user_input.len();
        // Unsafe block to rebuild the string from its raw pointer.
        // Note: No validation or sanitization is done on the input here.
        let unsanitized = unsafe {
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice)
        };
        // Directly inject the unsanitized input into the template.
        format!("{}{} </body></html>", self.template, unsanitized)
    }
}

fn main() {
    // In a real application, the input might come from user-provided parameters.
    // Here we take it from the command line or default to a benign string.
    let args: Vec<String> = std::env::args().collect();
    let user_data = if args.len() > 1 {
        args[1].clone()
    } else {
        // Default content (can be replaced with malicious input for testing)
        "Default Content".to_string()
    };

    // Wrap the Page in an Arc to safely share it across threads.
    let page = Arc::new(Page::new("<html><body>"));
    let page_clone = Arc::clone(&page);

    // Spawn a thread that renders the page with the supplied user data.
    let handle = thread::spawn(move || {
        // This unsafe-based method builds the HTML content without sanitizing user data.
        page_clone.assemble(&user_data)
    });

    // Wait for the thread to join and then print the resulting HTML.
    let result = handle.join().unwrap();
    println!("{}", result);
}