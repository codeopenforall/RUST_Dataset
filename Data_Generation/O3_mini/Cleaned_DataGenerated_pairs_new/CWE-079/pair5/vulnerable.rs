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
    fn build_page(&self, input: &str) -> String {
        let mut page = String::from("<html><body>");
        unsafe {
            let raw_ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let unescaped = String::from_utf8_lossy(slice);
            page.push_str(&format!("<div>{}</div>", unescaped));
        }
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
