use std::sync::{Arc, Mutex};
use std::thread;
struct Renderer {
    template: String,
}
impl Renderer {
    fn new() -> Self {
        Renderer { template: "<html><body>{}</body></html>".to_string() }
    }
    fn generate(&self, input: &str) -> String {
        let safe_input = html_escape(input);
        self.template.replace("{}", &safe_input)
    }
}
fn html_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(c),
        }
    }
    escaped
}
fn main() {
    let renderer = Arc::new(Mutex::new(Renderer::new()));
    let renderer_clone = Arc::clone(&renderer);
    let handle = thread::spawn(move || {
        let input = "<script>alert('XSS');</script>";
        let html = renderer_clone.lock().unwrap().generate(input);
        println!("{}", html);
    });
    handle.join().unwrap();
}
