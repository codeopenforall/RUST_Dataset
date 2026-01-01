use std::sync::Arc;
use std::thread;
fn escape_html(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '&'  => "&amp;".to_string(),
            '<'  => "&lt;".to_string(),
            '>'  => "&gt;".to_string(),
            '"'  => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            '/'  => "&#x2F;".to_string(),
            _    => c.to_string(),
        })
        .collect()
}
struct App {
}
impl App {
    fn new() -> Self {
        App {}
    }
    fn render(&self, input: &str) -> String {
        let header = "<html><body>Hello, ";
        let footer = "</body></html>";
        let safe_input = escape_html(input);
        let total_len = header.len() + safe_input.len() + footer.len();
        let mut buffer: Vec<u8> = Vec::with_capacity(total_len);
        unsafe {
            buffer.set_len(total_len);
            std::ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), header.len());
            std::ptr::copy_nonoverlapping(
                safe_input.as_ptr(),
                buffer.as_mut_ptr().add(header.len()),
                safe_input.len(),
            );
            std::ptr::copy_nonoverlapping(
                footer.as_ptr(),
                buffer.as_mut_ptr().add(header.len() + safe_input.len()),
                footer.len(),
            );
        }
        String::from_utf8(buffer).unwrap()
    }
    fn handle(&self, payload: String) -> String {
        self.render(&payload)
    }
}
fn main() {
    let app = Arc::new(App::new());
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        let attack_payload = "<script>alert('XSS');</script>";
        app_clone.handle(attack_payload.to_string())
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}
