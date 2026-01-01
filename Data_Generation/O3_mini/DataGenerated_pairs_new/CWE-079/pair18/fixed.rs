///////////////////////////////////////////////////////////////
// Fixed Code (CWE-079: Mitigation of Reflected XSS)
// This version properly escapes HTML special characters before embedding 
// untrusted input into the template, eliminating the XSS risk.
///////////////////////////////////////////////////////////////
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
        let safe = escape_html(input);
        format!("{}<div>{}</div>{}", self.header, safe, self.content)
    }
}

// Simple HTML escaping function to neutralize harmful characters
fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(ch),
        }
    }
    escaped
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