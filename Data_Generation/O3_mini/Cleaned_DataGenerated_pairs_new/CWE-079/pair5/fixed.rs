use std::sync::{Arc, Mutex};
use std::thread;
struct Renderer {
    content: Arc<Mutex<String>>,
}
impl Renderer {
    fn new() -> Self {
        Renderer {
            content: Arc::new(Mutex::new(String::new())),
        }
    }
    fn escape_html(&self, input: &str) -> String {
        input.chars()
            .map(|c| match c {
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '&' => "&amp;".to_string(),
                '"' => "&quot;".to_string(),
                '\''=> "&#x27;".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
    fn build_page(&self, input: &str) -> String {
        let mut page = String::from("<html><body>");
        let safe_input = self.escape_html(input);
        page.push_str(&format!("<div>{}</div>", safe_input));
        page.push_str("</body></html>");
        page
    }
    fn update_content(&self, text: String) {
        let shared = Arc::clone(&self.content);
        let handle = thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            *guard = text;
        });
        handle.join().unwrap();
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_input = if args.len() > 1 { &args[1] } else { "default" };
    let renderer = Renderer::new();
    let page = renderer.build_page(user_input);
    println!("{}", page);
}
