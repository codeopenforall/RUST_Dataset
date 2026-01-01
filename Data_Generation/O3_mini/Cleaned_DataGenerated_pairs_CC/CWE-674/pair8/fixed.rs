use std::env;
#[derive(Debug)]
pub struct Node {
    pub kind: char,
    pub children: Vec<Node>,
}
pub struct Parser {
    data: String,
    pos: usize,
}
impl Parser {
    pub fn new(data: String) -> Self {
        Parser { data, pos: 0 }
    }
    pub unsafe fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.data.len() {
            return None;
        }
        let ptr = self.data.as_ptr().add(self.pos);
        self.pos += 1;
        Some(*ptr as char)
    }
    pub fn parse_expr(&mut self, depth: usize) -> Result<Node, String> {
        const MAX_DEPTH: usize = 1000;
        if depth > MAX_DEPTH {
            return Err("Recursion depth limit exceeded".into());
        }
        let ch = unsafe { self.next_char() }.ok_or("Unexpected end of input")?;
        if ch == '(' {
            let mut children = Vec::new();
            loop {
                if self.pos < self.data.len() {
                    let peek = unsafe { *self.data.as_ptr().add(self.pos) as char };
                    if peek == ')' {
                        unsafe { self.next_char() };
                        break;
                    }
                } else {
                    return Err("Missing closing parenthesis".into());
                }
                let child = self.parse_expr(depth + 1)?;
                children.push(child);
            }
            Ok(Node { kind: '(', children })
        } else if ch == 'a' {
            Ok(Node { kind: 'a', children: vec![] })
        } else {
            Err(format!("Unexpected character: {}", ch))
        }
    }
}
// Public interface used for processing the input string.
pub fn process_input(input: &str) -> Result<Node, String> {
    let mut parser = Parser::new(input.to_owned());
    parser.parse_expr(0)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "(a)".to_string()
    };
    match process_input(&input) {
        Ok(node) => println!("Parsed successfully: {:?}", node),
        Err(err) => eprintln!("Error: {}", err),
    }
}
