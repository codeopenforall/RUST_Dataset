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
    fn render(&self, input: &str) -> String {
        let mut output = self.template.clone();
        let placeholder = "{{input}}";
        if let Some(pos) = output.find(placeholder) {
            unsafe {
                let ptr = output.as_mut_ptr();
                let bytes = input.as_bytes();
                for i in 0..bytes.len() {
                    *ptr.add(pos + i) = bytes[i];
                }
            }
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
