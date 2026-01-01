use std::thread;
use std::sync::{Arc, Mutex};
struct WebTemplate {
    header: String,
    footer: String,
}
impl WebTemplate {
    fn new() -> Self {
        WebTemplate {
            header: "<html><head><title>Demo</title></head><body>".to_string(),
            footer: "</body></html>".to_string(),
        }
    }
    fn render(&self, user_content: &str) -> String {
        let mut output = String::with_capacity(1024);
        unsafe {
            let buf = output.as_mut_vec();
            buf.extend_from_slice(self.header.as_bytes());
        }
        output.push_str(user_content);
        unsafe {
            let buf = output.as_mut_vec();
            buf.extend_from_slice(self.footer.as_bytes());
        }
        output
    }
}
fn process_request(user_input: &str) -> String {
    let template = WebTemplate::new();
    template.render(user_input)
}
fn main() {
    let shared_response = Arc::new(Mutex::new(String::new()));
    let input = if cfg!(test) { "<script>alert('XSS')</script>" } else { "<script>alert('XSS')</script>" };
    let shared_clone = Arc::clone(&shared_response);
    let handler = thread::spawn(move || {
        let response = process_request(input);
        let mut guard = shared_clone.lock().unwrap();
        *guard = response;
    });
    handler.join().expect("Thread panicked");
    let result = shared_response.lock().unwrap();
    println!("{}", *result);
}
