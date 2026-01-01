use std::iter::Peekable;
use std::str::Chars;
use std::sync::Arc;
use std::thread;
struct Parser;
impl Parser {
    fn new() -> Self {
        Parser
    }
    unsafe fn recursive_parse(&self, iter: &mut Peekable<Chars>) -> Result<(), String> {
        while let Some(&c) = iter.peek() {
            if c == '(' {
                iter.next();
                self.recursive_parse(iter)?;
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
            self.recursive_parse(&mut iter)
        }
    }
}
fn main() {
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
