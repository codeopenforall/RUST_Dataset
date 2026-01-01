//////////////////////
// Corrected Version
//////////////////////
use std::env;
use std::process;

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    // Recursive descent parser with a maximum recursion depth to prevent overflow.
    unsafe fn parse_recursive(&mut self, depth: u32) -> Result<(), String> {
        const MAX_DEPTH: u32 = 1024;
        if depth > MAX_DEPTH {
            return Err("Recursion too deep".into());
        }
        if self.pos >= self.input.len() {
            return Ok(());
        }
        let current = self.input.as_bytes()[self.pos];
        if current == b'(' {
            self.pos += 1;
            // Recursive call now includes a depth counter.
            self.parse_recursive(depth + 1)?;
            if self.pos < self.input.len() && self.input.as_bytes()[self.pos] == b')' {
                self.pos += 1;
                Ok(())
            } else {
                Err("Missing closing ')'".into())
            }
        } else {
            self.pos += 1;
            Ok(())
        }
    }

    // Public API that initiates the recursive parsing with depth counter starting at 0.
    pub unsafe fn execute(input: &str) -> Result<(), String> {
        let mut parser = Parser {
            input: input.to_string(),
            pos: 0,
        };
        parser.parse_recursive(0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = if args.len() > 1 { &args[1] } else { "(a)" };
    unsafe {
        match Parser::execute(data) {
            Ok(_) => println!("Parsing succeeded."),
            Err(e) => {
                eprintln!("Parsing failed: {}", e);
                process::exit(1);
            }
        }
    }
}