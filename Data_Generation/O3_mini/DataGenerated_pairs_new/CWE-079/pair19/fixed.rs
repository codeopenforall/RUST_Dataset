//////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust program implements a similar HTML page renderer, but now the user-supplied input is
// properly sanitized before it is embedded into the HTML template. An HTML escaping function is applied
// to neutralize any potential Cross-Site Scripting vectors. Concurrency and thread usage remain.
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

    // Escapes special HTML characters in the user input.
    fn escape_html(input: &str) -> String {
        input.chars().map(|c| {
            match c {
                '&'  => "&amp;".to_string(),
                '<'  => "&lt;".to_string(),
                '>'  => "&gt;".to_string(),
                '"'  => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _    => c.to_string(),
            }
        }).collect::<String>()
    }

    // This method now safely assembles the HTML by escaping user input.
    pub fn assemble(&self, user_input: &str) -> String {
        // Even though we use an unsafe block to mimic similar structure, the raw bytes are converted,
        // but then the resulting string is passed through an escaping function.
        let ptr = user_input.as_ptr();
        let len = user_input.len();
        let raw_input = unsafe {
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice)
        };
        // Escape HTML special characters.
        let safe_input = Self::escape_html(raw_input);
        format!("{}{} </body></html>", self.template, safe_input)
    }
}

fn main() {
    // Fetch user data from command line arguments (or use default benign input).
    let args: Vec<String> = std::env::args().collect();
    let user_data = if args.len() > 1 {
        args[1].clone()
    } else {
        "Default Content".to_string()
    };

    // Wrap the Page in an Arc for safe sharing.
    let page = Arc::new(Page::new("<html><body>"));
    let page_clone = Arc::clone(&page);

    // Spawn a thread that renders the page with sanitized user input.
    let handle = thread::spawn(move || {
        page_clone.assemble(&user_data)
    });

    // Wait for the thread and print the safely rendered HTML.
    let result = handle.join().unwrap();
    println!("{}", result);
}