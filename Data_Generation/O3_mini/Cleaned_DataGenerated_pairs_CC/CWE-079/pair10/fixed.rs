use std::sync::{Arc, Mutex};
use std::thread;
struct Renderer {
    template: String,
}
impl Renderer {
    pub fn new() -> Self {
        Renderer { template: "<html><body>{content}</body></html>".to_string() }
    }
    fn escape(input: &str) -> String {
        input.chars().map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        }).collect()
    }
    pub fn render(&self, user_input: &str) -> String {
        let safe_input = Self::escape(user_input);
        let placeholder = "{content}";
        if let Some(pos) = self.template.find(placeholder) {
            let before = &self.template[..pos];
            let after = &self.template[pos + placeholder.len()..];
            format!("{}{}{}", before, safe_input, after)
        } else {
            self.template.clone()
        }
    }
}
fn main() {
    let renderer = Arc::new(Mutex::new(Renderer::new()));
    let untrusted_input = "<script>alert('XSS');</script>";
    let mut handles = Vec::new();
    for _ in 0..5 {
        let render_clone = Arc::clone(&renderer);
        let input_clone = untrusted_input.to_string();
        let handle = thread::spawn(move || {
            let guard = render_clone.lock().unwrap();
            let output = guard.render(&input_clone);
            output
        });
        handles.push(handle);
    }
    for handle in handles {
        let output = handle.join().unwrap();
        println!("{}", output);
    }
}
