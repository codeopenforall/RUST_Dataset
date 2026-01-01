use std::ptr;
const MAX_DEPTH: usize = 100;
struct Parser {
    data: *const u8,
    len: usize,
    pos: usize,
}
impl Parser {
    fn new(input: &str) -> Self {
        let bytes = input.as_bytes();
        Parser {
            data: bytes.as_ptr(),
            len: bytes.len(),
            pos: 0,
        }
    }
    fn next_byte(&mut self) -> Option<u8> {
        if self.pos < self.len {
            let byte = unsafe { ptr::read(self.data.add(self.pos)) };
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }
    fn parse(&mut self, depth: usize) -> Result<(), &'static str> {
        if depth > MAX_DEPTH {
            return Err("Maximum recursion depth exceeded");
        }
        match self.next_byte() {
            Some(b'(') => {
                self.parse(depth + 1)?; 
                match self.next_byte() {
                    Some(b')') => Ok(()),
                    _ => Err("Missing closing parenthesis"),
                }
            },
            Some(b')') => Err("Unexpected closing parenthesis"),
            Some(_) => self.parse(depth),
            None => Ok(()),
        }
    }
}
fn process(input: &str) -> Result<(), &'static str> {
    let mut p = Parser::new(input);
    p.parse(0)
}
fn main() {
    let input = "(())";
    match process(input) {
        Ok(_) => println!("Parsed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
