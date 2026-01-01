use std::sync::{Arc, Mutex};
use std::thread;
fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '&' => "&amp;".to_owned(),
            '"' => "&quot;".to_owned(),
            '\'' => "&#x27;".to_owned(),
            _ => c.to_string()
        }
    }).collect()
}
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
        let safe_input = escape_html(input);
        format!("<html><body>Hello, {}!</body></html>", safe_input)
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
