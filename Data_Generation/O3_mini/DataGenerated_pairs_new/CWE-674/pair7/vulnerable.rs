///////////////////////////
// Vulnerable Version Code
///////////////////////////
#![allow(dead_code)]
use std::env;
use std::vec::Vec;

struct Node {
    children: Vec<Node>,
}

impl Node {
    // Using an unsafe constructor to mimic low-level API usage.
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
    
    // Entry point for parsing that delegates to a recursive function.
    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested()
    }

    // Recursive descent parser without any recursion depth limit.
    fn parse_nested(&mut self) -> Result<Node, &'static str> {
        if self.pos >= self.input.len() {
            return Err("Unexpected end of input");
        }
        let mut node;
        // Unsafe block used for low-level construction of the node.
        unsafe {
            node = Node::new();
        }
        // Check for an opening parenthesis and process nested expressions.
        if self.input[self.pos] == '(' {
            self.pos += 1;
            // Recurse for every nested '(' encountered.
            while self.pos < self.input.len() && self.input[self.pos] == '(' {
                // Unbounded recursion: no limit is imposed on the depth.
                node.children.push(self.parse_nested()?);
            }
            // Expect a matching closing parenthesis.
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
    // For deep nested inputs the recursion might overflow the stack.
    let mut parser = Parser::new(&args[1]);
    match parser.parse_expr() {
        Ok(_node) => println!("Parsed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}