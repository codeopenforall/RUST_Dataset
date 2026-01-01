use std::sync::{Arc, Mutex};
use std::thread;
use std::str;
struct Renderer {
    tpl: String,
}
impl Renderer {
    fn new(template: &str) -> Self {
        Renderer {
            tpl: template.to_owned(),
        }
    }
    fn render(&self, input: &str) -> String {
        let mut output = self.tpl.clone();
        unsafe {
            let bytes = input.as_bytes();
            let untrusted = str::from_utf8_unchecked(bytes);
            output = output.replace("{user_data}", untrusted);
        }
        output
    }
}
fn concurrent_process(renderer: Arc<Mutex<Renderer>>, input: String) -> String {
    let handle = thread::spawn(move || {
        let renderer = renderer.lock().unwrap();
        renderer.render(&input)
    });
    handle.join().unwrap()
}
fn main() {
    let tpl = "<html><body>Hello, {user_data}!</body></html>";
    let renderer = Arc::new(Mutex::new(Renderer::new(tpl)));
    let user_input = "<script>alert('XSS');</script>".to_string();
    let result = concurrent_process(renderer, user_input);
    println!("Output: {}", result);
}
