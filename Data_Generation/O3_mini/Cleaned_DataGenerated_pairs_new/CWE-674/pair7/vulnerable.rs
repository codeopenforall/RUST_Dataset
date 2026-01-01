#![allow(dead_code)]
use std::env;
use std::vec::Vec;
struct Node {
    children: Vec<Node>,
}
impl Node {
    unsafe fn new() -> Self {
        Node { children: Vec::new() }
    }
}
struct Parser {
    input: Vec<char>,
    pos: usize,
}
impl Parser {
    fn new(input_str: &str) -> Self {
        Parser {
            input: input_str.chars().collect(),
            pos: 0,
        }
    }
    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested()
    }
    fn parse_nested(&mut self) -> Result<Node, &'static str> {
        if self.pos >= self.input.len() {
            return Err("Unexpected end of input");
        }
        let mut node;
        unsafe {
            node = Node::new();
        }
        if self.input[self.pos] == '(' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos] == '(' {
                node.children.push(self.parse_nested()?);
            }
            if self.pos < self.input.len() && self.input[self.pos] == ')' {
                self.pos += 1;
            } else {
                return Err("Missing closing parenthesis");
            }
        }
        Ok(node)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let mut parser = Parser::new(&args[1]);
    match parser.parse_expr() {
        Ok(_node) => println!("Parsed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
