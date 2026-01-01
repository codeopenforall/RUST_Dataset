////////////////////////////////////////////////////////////////////////////////////////////////////
// This version corrects the XSS vulnerability by properly escaping the user provided input
// before embedding it into the HTML response. It completely removes the unsafe memory copy error,
// relying instead on safe string handling and escaping functions.
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct Renderer {}

impl Renderer {
    fn new() -> Self {
        Self {}
    }

    // Escapes special HTML characters to ensure the output is safe.
    fn escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\"', "&quot;")
            .replace('\'', "&#x27;")
    }

    // Returns an HTML string that safely embeds an escaped version of the input.
    fn render(&self, input: &str) -> String {
        let safe_input = self.escape(input);
        let mut html = String::new();
        html.push_str("<html><body>");
        html.push_str("User input: ");
        // Safe rendering: the input is escaped before being inserted into the HTML.
        html.push_str(&safe_input);
        html.push_str("</body></html>");
        html
    }
}

fn main() {
    let renderer = Arc::new(Renderer::new());
    // Obtain user input from command-line arguments.
    let input = std::env::args().nth(1).unwrap_or_else(|| String::new());
    let r = renderer.clone();
    let handle = thread::spawn(move || {
        let output = r.render(&input);
        println!("{}", output);
    });
    handle.join().unwrap();
}