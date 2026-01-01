use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
fn render(input: &str) -> String {
    let unsafe_str = unsafe { std::str::from_utf8_unchecked(input.as_bytes()) };
    format!("<html><body><h1>Welcome {}</h1></body></html>", unsafe_str)
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
