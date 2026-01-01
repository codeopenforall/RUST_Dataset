use std::sync::{Arc, Mutex};
use std::thread;
trait Template {
    fn render(&self, input: &str) -> String;
}
struct Page {
    header: String,
    content: String,
}
impl Template for Page {
    fn render(&self, input: &str) -> String {
        let dangerous = unsafe {
            let raw = input.as_bytes();
            std::str::from_utf8_unchecked(raw)
        };
        format!("{}<div>{}</div>{}", self.header, dangerous, self.content)
    }
}
fn process(input: &str, page: Arc<Mutex<Page>>) -> String {
    let result = {
        let guard = page.lock().unwrap();
        guard.render(input)
    };
    result
}
fn main() {
    let page = Arc::new(Mutex::new(Page {
        header: "<html><head><title>Test</title></head><body>".to_string(),
        content: "</body></html>".to_string(),
    }));
    let injection = "<script>alert('XSS');</script>";
    let mut threads = vec![];
    for _ in 0..4 {
        let copy = Arc::clone(&page);
        let text = injection.to_string();
        threads.push(thread::spawn(move || process(&text, copy)));
    }
    for handle in threads {
        let html = handle.join().unwrap();
        println!("{}", html);
    }
}
