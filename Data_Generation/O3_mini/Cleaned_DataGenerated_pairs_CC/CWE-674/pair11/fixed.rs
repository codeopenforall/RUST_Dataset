use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
const MAX_DEPTH: usize = 1000; 
fn safe_rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("Maximum recursion depth exceeded");
    }
    if s.is_empty() {
        return Ok(depth);
    }
    let first = s.as_bytes()[0];
    if first == b'(' {
        safe_rec_parse(&s[1..], depth + 1)
    } else if first == b')' {
        if depth == 0 {
            return Err("Unbalanced parentheses");
        }
        safe_rec_parse(&s[1..], depth - 1)
    } else {
        Err("Invalid character")
    }
}
fn parse_nested(s: &str) -> Result<usize, &'static str> {
    safe_rec_parse(s, 0)
}
fn worker(input: String) {
    thread::spawn(move || {
        let counter = Arc::new(Mutex::new(0));
        let result = parse_nested(&input);
        let count = counter.lock().unwrap();
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
