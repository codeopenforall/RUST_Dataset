use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
fn html_escape(input: &str) -> String {
    input.chars().fold(String::new(), |mut acc, ch| {
        match ch {
            '<' => acc.push_str("&lt;"),
            '>' => acc.push_str("&gt;"),
            '&' => acc.push_str("&amp;"),
            '"' => acc.push_str("&quot;"),
            '\'' => acc.push_str("&#x27;"),
            _ => acc.push(ch),
        }
        acc
    })
}
struct Processor {
    data: Arc<Mutex<String>>,
}
impl Processor {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(String::new())),
        }
    }
    fn process(&self, input: &str) -> String {
        let mut result = String::new();
        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let s = std::str::from_utf8(slice).unwrap();
            let safe = html_escape(s);
            result.push_str("Hello, ");
            result.push_str(&safe);
        }
        result
    }
    fn run(&self, processed: String) -> String {
        let arc = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            let mut guard = arc.lock().unwrap();
            *guard = format!("<html><body>{}</body></html>", processed);
            guard.clone()
        });
        handle.join().unwrap()
    }
}
pub fn process_input(input: &str) -> String {
    let processor = Processor::new();
    processor.run(processor.process(input))
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let raw = if args.len() > 1 { args[1].clone() } else { String::from("default") };
    let output = process_input(&raw);
    println!("{}", output);
}
