use std::sync::{Arc, Mutex};
use std::thread;
struct Renderer {
    data: Arc<Mutex<String>>,
}
impl Renderer {
    fn new() -> Self {
        Renderer {
            data: Arc::new(Mutex::new(String::new())),
        }
    }
    fn render(&self, input: &str) -> String {
        unsafe {
            let raw_input: &str = std::str::from_utf8_unchecked(input.as_bytes());
            format!("<html><body>Hello, {}!</body></html>", raw_input)
        }
    }
}
fn main() {
    let renderer = Renderer::new();
    let inputs = vec!["Alice", "<script>alert('XSS')</script>"];
    let threads: Vec<_> = inputs
        .into_iter()
        .map(|text| {
            let clone = renderer.data.clone();
            let instance = Renderer { data: clone };
            let payload = text.to_string();
            thread::spawn(move || {
                instance.render(&payload)
            })
        })
        .collect();
    for t in threads {
        let result = t.join().expect("Thread panicked");
        let mut locked = renderer.data.lock().unwrap();
        *locked = result.clone();
        println!("{}", result);
    }
}
