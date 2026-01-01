use std::sync::{Arc, Mutex};
use std::thread;
struct PageTemplate {
    content: String,
}
impl PageTemplate {
    fn new() -> Self {
        PageTemplate {
            content: "<html><body>{content}</body></html>".to_string(),
        }
    }
    unsafe fn render(&self, user_input: *const u8, len: usize) -> String {
        let slice = std::slice::from_raw_parts(user_input, len);
        let input_str = std::str::from_utf8_unchecked(slice);
        self.content.replace("{content}", input_str)
    }
}
fn process_request(input: String) -> String {
    let template = Arc::new(PageTemplate::new());
    let user_data = Arc::new(input);
    let result = Arc::new(Mutex::new(String::new()));
    let template_clone = Arc::clone(&template);
    let user_clone = Arc::clone(&user_data);
    let result_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let raw = user_clone.as_ptr();
        let len = user_clone.len();
        let page = unsafe { template_clone.render(raw, len) };
        let mut res = result_clone.lock().unwrap();
        *res = page;
    });
    handle.join().unwrap();
    Arc::try_unwrap(result).ok().unwrap().into_inner().unwrap()
}
fn main() {
    let malicious = "<script>alert('XSS');</script>".to_string();
    let response = process_request(malicious);
    println!("{}", response);
}
