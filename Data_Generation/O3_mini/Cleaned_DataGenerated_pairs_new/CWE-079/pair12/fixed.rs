use std::sync::{Arc, Mutex};
use std::thread;
struct TemplateEngine {
    template: String,
}
impl TemplateEngine {
    fn new(template: &str) -> Self {
        TemplateEngine {
            template: template.to_string(),
        }
    }
    fn escape_html(input: &str) -> String {
        input
            .chars()
            .map(|ch| match ch {
                '&' => "&amp;".to_string(),
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _ => ch.to_string(),
            })
            .collect()
    }
    fn render(&self, input: &str) -> String {
        let mut output = self.template.clone();
        let placeholder = "{{input}}";
        if output.contains(placeholder) {
            let escaped = Self::escape_html(input);
            output = output.replace(placeholder, &escaped);
        }
        output
    }
}
fn main() {
    let engine = Arc::new(Mutex::new(TemplateEngine::new(
        "<html><body>User: {{input}}</body></html>",
    )));
    let engine_clone = engine.clone();
    let handle = thread::spawn(move || {
        let eng = engine_clone.lock().unwrap();
        let user_input = "<script>alert('xss');</script>";
        eng.render(user_input)
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}
