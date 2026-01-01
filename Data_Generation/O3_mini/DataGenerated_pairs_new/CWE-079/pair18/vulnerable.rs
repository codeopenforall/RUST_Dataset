///////////////////////////////////////////////////////////////
// Vulnerable Code (CWE-079: Reflected XSS through unescaped input)
// This version directly embeds untrusted input into HTML using unsafe 
// string conversion and concurrent processing.
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
        // Unsafe conversion bypassing UTF-8 checks (mimics low-level operations)
        let dangerous = unsafe {
            let raw = input.as_bytes();
            std::str::from_utf8_unchecked(raw)
        };
        // Vulnerability: untrusted input is inserted directly into the HTML template without escaping
        format!("{}<div>{}</div>{}", self.header, dangerous, self.content)
    }
}

fn process(input: &str, page: Arc<Mutex<Page>>) -> String {
    // Locking simulates shared usage in a concurrent context
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

    // Spawn threads to concurrently produce page responses
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