use std::sync::Arc;
use std::thread;
struct Renderer {}
impl Renderer {
    fn new() -> Self {
        Self {}
    }
    fn escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\"', "&quot;")
            .replace('\'', "&#x27;")
    }
    fn render(&self, input: &str) -> String {
        let safe_input = self.escape(input);
        let mut html = String::new();
        html.push_str("<html><body>");
        html.push_str("User input: ");
        html.push_str(&safe_input);
        html.push_str("</body></html>");
        html
    }
}
fn main() {
    let renderer = Arc::new(Renderer::new());
    let input = std::env::args().nth(1).unwrap_or_else(|| String::new());
    let r = renderer.clone();
    let handle = thread::spawn(move || {
        let output = r.render(&input);
        println!("{}", output);
    });
    handle.join().unwrap();
}
