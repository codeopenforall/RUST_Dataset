use std::thread;
const MAX_DEPTH: usize = 100;
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
    fn descend(&mut self, depth: usize) -> Result<(), String> {
        if depth > MAX_DEPTH {
            return Err("Maximum recursion depth reached".into());
        }
        if self.pos >= self.data.len() {
            return Ok(());
        }
        let ptr = self.data.as_ptr().wrapping_add(self.pos);
        let ch = unsafe { *ptr as char };
        match ch {
            '(' => {
                self.pos += 1;
                self.descend(depth + 1)?;
                if self.pos < self.data.len() && self.data[self.pos] as char == ')' {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err("Missing closing parenthesis".into())
                }
            }
            _ => {
                self.pos += 1;
                self.descend(depth)
            }
        }
    }
}
fn process(input: &str) -> Result<(), String> {
    let mut parser = Parser::new(input);
    parser.descend(0)
}
fn parser_run(input: &str) -> Result<(), String> {
    process(input)
}
fn main() {
    let input = "(".repeat(1000) + &")".repeat(1000);
    let handle = thread::spawn(move || {
        match parser_run(&input) {
            Ok(()) => println!("Parsing succeeded"),
            Err(e) => println!("Parsing failed: {}", e),
        }
    });
    let _ = handle.join();
}
