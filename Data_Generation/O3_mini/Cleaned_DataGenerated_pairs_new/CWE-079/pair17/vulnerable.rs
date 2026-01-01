use std::sync::{Arc, Mutex};
use std::thread;
struct Renderer {
    buffer: Arc<Mutex<Vec<u8>>>,
}
impl Renderer {
    fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(vec![0u8; 10])),
        }
    }
    fn render(&self, input: &str) -> String {
        {
            let mut guard = self.buffer.lock().unwrap();
            unsafe {
                let dest = guard.as_mut_ptr();
                let src = input.as_bytes().as_ptr();
                std::ptr::copy_nonoverlapping(src, dest, input.len());
            }
        }
        let mut html = String::new();
        html.push_str("<html><body>");
        html.push_str("User input: ");
        html.push_str(input);
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
