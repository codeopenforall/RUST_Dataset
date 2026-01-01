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
    fn render(&self, user_input: &str) -> String {
        let safe_input = escape_html(user_input);
        self.content.replace("{content}", &safe_input)
    }
}
fn escape_html(input: &str) -> String {
    input.chars().map(|c| match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '&' => "&amp;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&#x27;".to_string(),
        _ => c.to_string(),
    }).collect()
}
fn process_request(input: String) -> String {
    let template = Arc::new(PageTemplate::new());
    let user_data = Arc::new(input);
    let result = Arc::new(Mutex::new(String::new()));
    let template_clone = Arc::clone(&template);
    let user_clone = Arc::clone(&user_data);
    let result_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let page = template_clone.render(&user_clone);
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
