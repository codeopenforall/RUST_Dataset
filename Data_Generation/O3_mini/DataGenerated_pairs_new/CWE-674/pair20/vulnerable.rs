//////////////////////
// Vulnerable Version
//////////////////////
use std::env;
use std::process;

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    // Recursive descent parser without depth control.
    // Unsafe block is used to directly access byte slices.
    unsafe fn parse_recursive(&mut self) -> Result<(), String> {
        if self.pos >= self.input.len() {
            return Ok(());
        }
        // Get current byte unsafely.
        let current = self.input.as_bytes()[self.pos];
        if current == b'(' {
            self.pos += 1;
            // Uncontrolled recursion: no depth limit check.
            self.parse_recursive()?; // <-- Vulnerability: no depth restriction.
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

    // Public API to execute the parser on an input.
    pub unsafe fn execute(input: &str) -> Result<(), String> {
        let mut parser = Parser {
            input: input.to_string(),
            pos: 0,
        };
        parser.parse_recursive()
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