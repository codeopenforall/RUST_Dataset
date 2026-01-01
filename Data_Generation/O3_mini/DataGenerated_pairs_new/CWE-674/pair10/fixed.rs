////////////////////////////////////////////////////////
// Corrected Code Sample that fixes uncontrolled recursion vulnerability
// The fix introduces a maximum recursion depth limit to prevent stack overflow.
// It uses an extra depth parameter to track the recursion level and returns an error if exceeded.
////////////////////////////////////////////////////////
use std::iter::Peekable;
use std::str::Chars;
use std::sync::Arc;
use std::thread;

struct Parser;

impl Parser {
    fn new() -> Self {
        Parser
    }

    // Define a maximum recursion depth to limit recursive calls.
    const MAX_DEPTH: usize = 512;

    // Unsafe recursive parser with an extra depth parameter.
    unsafe fn recursive_parse(&self, iter: &mut Peekable<Chars>, depth: usize) -> Result<(), String> {
        if depth > Self::MAX_DEPTH {
            return Err("Recursion depth limit exceeded".to_string());
        }
        while let Some(&c) = iter.peek() {
            if c == '(' {
                iter.next();
                // Increment depth on each recursive call
                self.recursive_parse(iter, depth + 1)?;
            } else if c == ')' {
                iter.next();
                return Ok(());
            } else {
                iter.next();
            }
        }
        Ok(())
    }

    fn parse(&self, input: &str) -> Result<(), String> {
        let mut iter = input.chars().peekable();
        unsafe {
            self.recursive_parse(&mut iter, 0)
        }
    }
}

fn main() {
    // Even though attacker input remains the same, the parser now protects against stack overflow.
    let attacker_input = "(".repeat(1000) + &")".repeat(1000);
    let parser = Parser::new();
    let shared_parser = Arc::new(parser);

    let handles: Vec<_> = (0..4).map(|_| {
        let sp = Arc::clone(&shared_parser);
        let input_clone = attacker_input.clone();
        thread::spawn(move || {
            sp.parse(&input_clone)
        })
    }).collect();

    for handle in handles {
        match handle.join() {
            Ok(Ok(())) => println!("Thread finished execution"),
            Ok(Err(e)) => println!("Error: {}", e),
            Err(_) => println!("Thread panicked"),
        }
    }
}