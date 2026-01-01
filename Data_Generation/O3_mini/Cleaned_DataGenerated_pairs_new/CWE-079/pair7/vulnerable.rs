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
        unsafe {
            let bytes = input.as_bytes();
            let untrusted = std::str::from_utf8_unchecked(bytes);
            self.template.replace("{}", untrusted)
        }
    }
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
