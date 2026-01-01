use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            '/' => escaped.push_str("&#x2F;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
fn render(input: &str) -> String {
    let safe_str = escape_html(input);
    format!("<html><body><h1>Welcome {}</h1></body></html>", safe_str)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 { args[1].clone() } else { String::new() };
    let shared = Arc::new(Mutex::new(user_input));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let input = shared_clone.lock().unwrap();
        let page = render(&input);
        println!("{}", page);
    });
    handle.join().unwrap();
}
