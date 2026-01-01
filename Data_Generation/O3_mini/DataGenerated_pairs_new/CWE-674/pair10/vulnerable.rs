////////////////////////////////////////////////////////
// Vulnerable Code Sample targeting uncontrolled recursion vulnerability (CWE-674)
// This code uses unsafe blocks and recursive descent parsing without depth limits.
// It creates multiple threads that each parse a deeply nested string,
// which may lead to stack overflow.
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

    // Unsafe recursive descent parser without any depth limit
    unsafe fn recursive_parse(&self, iter: &mut Peekable<Chars>) -> Result<(), String> {
        while let Some(&c) = iter.peek() {
            if c == '(' {
                iter.next();
                // Uncontrolled recursion: no check for maximum recursion depth.
                self.recursive_parse(iter)?;
            } else if c == ')' {
                iter.next();
                return Ok(());
            } else {
                // Skip any other character
                iter.next();
            }
        }
        Ok(())
    }

    fn parse(&self, input: &str) -> Result<(), String> {
        let mut iter = input.chars().peekable();
        unsafe {
            self.recursive_parse(&mut iter)
        }
    }
}

fn main() {
    // Attacker controlled deeply nested input
    let attacker_input = "(".repeat(1000) + &")".repeat(1000);
    let parser = Parser::new();
    let shared_parser = Arc::new(parser);

    let handles: Vec<_> = (0..4).map(|_| {
        let sp = Arc::clone(&shared_parser);
        let input_clone = attacker_input.clone();
        thread::spawn(move || {
            // This recursive call may overflow the stack under attacker's input
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