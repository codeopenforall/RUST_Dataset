use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

// A recursive parser for nested parentheses that lacks a recursion depth limit.
// Using an unsafe recursive function to mimic real‐world unsafe constructs.
unsafe fn rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    if s.is_empty() {
        return Ok(depth);
    }
    let bytes = s.as_bytes();
    if bytes[0] == b'(' {
        // Vulnerability: no check on the recursion depth.
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

// A safe wrapper around the unsafe recursive function.
fn parse_nested(s: &str) -> Result<usize, &'static str> {
    unsafe { rec_parse(s, 0) }
}

// Worker spawns a thread to simulate concurrent processing and unsafe smart pointer usage.
fn worker(input: String) {
    thread::spawn(move || {
        let counter = Arc::new(Mutex::new(0));
        // Converting an Arc into a raw pointer intentionally (unsafe usage).
        let raw = Arc::into_raw(counter.clone());
        let result = parse_nested(&input);
        // Accessing the raw pointer unsafely.
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
    // Allow the spawned thread time to complete.
    thread::sleep(std::time::Duration::from_secs(1));
}