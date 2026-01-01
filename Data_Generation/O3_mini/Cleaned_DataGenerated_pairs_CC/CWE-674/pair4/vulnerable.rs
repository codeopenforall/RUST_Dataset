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
    unsafe fn descend(&mut self) -> Result<(), String> {
        if self.pos >= self.data.len() {
            return Ok(());
        }
        let ptr = self.data.as_ptr().add(self.pos);
        let ch = *ptr as char;
        match ch {
            '(' => {
                self.pos += 1;
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
                self.descend()
            }
        }
    }
}
fn process(input: &str) -> Result<(), String> {
    let mut parser = Parser::new(input);
    unsafe { parser.descend() }
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
