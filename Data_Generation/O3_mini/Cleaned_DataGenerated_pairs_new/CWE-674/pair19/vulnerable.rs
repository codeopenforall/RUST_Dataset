use std::ptr;
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }
    fn current(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
    fn consume(&mut self) -> Option<char> {
        if let Some(ch) = self.current() {
            unsafe {
                let _slice = self.input.as_bytes();
                let len = ch.len_utf8();
                self.pos += len;
            }
            Some(ch)
        } else {
            None
        }
    }
    fn parse_expression(&mut self) -> bool {
        if let Some(ch) = self.current() {
            if ch == '(' {
                self.consume();
                let res = self.parse_expression();
                if self.current() == Some(')') {
                    self.consume();
                    return res;
                }
            }
        }
        true
    }
}
fn main() {
    let mut input = String::new();
    for _ in 0..10000 {
        input.push('(');
    }
    for _ in 0..10000 {
        input.push(')');
    }
    let mut parser = Parser::new(&input);
    let valid = parser.parse_expression();
    println!("Result: {}", valid);
}
