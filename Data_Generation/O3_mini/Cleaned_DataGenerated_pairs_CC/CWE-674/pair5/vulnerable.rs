use std::ptr;
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
    fn parse(&mut self) -> Result<(), &'static str> {
        match self.next_byte() {
            Some(b'(') => {
                self.parse()?; 
                match self.next_byte() {
                    Some(b')') => Ok(()),
                    _ => Err("Missing closing parenthesis"),
                }
            },
            Some(b')') => Err("Unexpected closing parenthesis"),
            Some(_) => self.parse(), 
            None => Ok(()),
        }
    }
}
fn process(input: &str) -> Result<(), &'static str> {
    let mut p = Parser::new(input);
    p.parse()
}
fn main() {
    let input = "(())";
    match process(input) {
        Ok(_) => println!("Parsed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
