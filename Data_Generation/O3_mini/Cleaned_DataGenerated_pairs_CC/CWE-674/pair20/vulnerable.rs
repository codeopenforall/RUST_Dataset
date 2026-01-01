use std::env;
use std::process;
pub struct Parser {
    input: String,
    pos: usize,
}
impl Parser {
    unsafe fn parse_recursive(&mut self) -> Result<(), String> {
        if self.pos >= self.input.len() {
            return Ok(());
        }
        let current = self.input.as_bytes()[self.pos];
        if current == b'(' {
            self.pos += 1;
            self.parse_recursive()?; 
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
