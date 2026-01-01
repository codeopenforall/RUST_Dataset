//////////////////////////////
// Fixed Version Code       //
//////////////////////////////
use std::thread;
use std::sync::{Arc, Mutex};

struct WebTemplate {
    header: String,
    footer: String,
}

impl WebTemplate {
    fn new() -> Self {
        // Construct a basic HTML template.
        WebTemplate {
            header: "<html><head><title>Demo</title></head><body>".to_string(),
            footer: "</body></html>".to_string(),
        }
    }

    fn render(&self, user_content: &str) -> String {
        let mut output = String::with_capacity(1024);
        // Safely insert the header.
        output.push_str(&self.header);

        // Escape user input to neutralize any embedded HTML or script tags.
        output.push_str(&escape_html(user_content));

        // Safely insert the footer.
        output.push_str(&self.footer);
        output
    }
}

// A helper function to escape HTML special characters.
fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '<'  => "&lt;".to_string(),
            '>'  => "&gt;".to_string(),
            '&'  => "&amp;".to_string(),
            '"'  => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _    => c.to_string(),
        }
    }).collect::<String>()
}

fn process_request(user_input: &str) -> String {
    let template = WebTemplate::new();
    template.render(user_input)
}

fn main() {
    // Simulate processing a web request containing untrusted input.
    let shared_response = Arc::new(Mutex::new(String::new()));
    // Simulated malicious input intended to trigger XSS.
    let input = if cfg!(test) { "<script>alert('XSS')</script>" } else { "<script>alert('XSS')</script>" };
    let shared_clone = Arc::clone(&shared_response);

    let handler = thread::spawn(move || {
        let response = process_request(input);
        let mut guard = shared_clone.lock().unwrap();
        *guard = response;
    });
    handler.join().expect("Thread panicked");

    let result = shared_response.lock().unwrap();
    println!("{}", *result);
}