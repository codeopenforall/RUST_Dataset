use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
unsafe fn rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    if s.is_empty() {
        return Ok(depth);
    }
    let bytes = s.as_bytes();
    if bytes[0] == b'(' {
        rec_parse(&s[1..], depth + 1)
    } else if bytes[0] == b')' {
        if depth == 0 {
            return Err("Unbalanced parentheses");
        }
        rec_parse(&s[1..], depth - 1)
    } else {
        Err("Invalid character")
    }
}
fn parse_nested(s: &str) -> Result<usize, &'static str> {
    unsafe { rec_parse(s, 0) }
}
fn worker(input: String) {
    thread::spawn(move || {
        let counter = Arc::new(Mutex::new(0));
        let raw = Arc::into_raw(counter.clone());
        let result = parse_nested(&input);
        let count = unsafe { (*raw).lock().unwrap() };
        println!("Result: {:?}, counter: {}", result, *count);
    });
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <nested parentheses string>");
        return;
    }
    let input = args[1].clone();
    worker(input);
    thread::sleep(std::time::Duration::from_secs(1));
}
