///////////////////////////////////////////////
// Vulnerable Rust Code
///////////////////////////////////////////////
use std::thread;

struct Parser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            data: input.as_bytes(),
            pos: 0,
        }
    }

    // UNSAFE recursive descent parsing without depth limits.
    unsafe fn descend(&mut self) -> Result<(), String> {
        if self.pos >= self.data.len() {
            return Ok(());
        }
        // Simulate low-level pointer usage.
        let ptr = self.data.as_ptr().add(self.pos);
        let ch = *ptr as char;
        match ch {
            '(' => {
                self.pos += 1;
                // Recursive call without checking the stack consumption.
                self.descend()?;
                if self.pos < self.data.len() && self.data[self.pos] as char == ')' {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err("Missing closing parenthesis".into())
                }
            }
            _ => {
                self.pos += 1;
                // Continue processing.
                self.descend()
            }
        }
    }
}

fn process(input: &str) -> Result<(), String> {
    let mut parser = Parser::new(input);
    // Unsafe block to execute the recursive descent.
    unsafe { parser.descend() }
}

fn parser_run(input: &str) -> Result<(), String> {
    process(input)
}

fn main() {
    // Construct a deeply nested input triggering uncontrolled recursion.
    let input = "(".repeat(1000) + &")".repeat(1000);
    // Launch parsing in a concurrent thread.
    let handle = thread::spawn(move || {
        match parser_run(&input) {
            Ok(()) => println!("Parsing succeeded"),
            Err(e) => println!("Parsing failed: {}", e),
        }
    });
    let _ = handle.join();
}