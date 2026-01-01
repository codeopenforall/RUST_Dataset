////////////////////////////////////////////////////////////////////////////////////////////////////
// This code implements a recursive descent parser that builds a tree structure from an input
// string. It uses an unsafe block to read input bytes directly via raw pointers. However, it
// fails to enforce any limits on the recursion depth while processing nested expressions,
// opening a path for uncontrolled recursion (CWE-674) that may lead to a stack overflow when
// processing attacker-controlled input with very deep nesting.
// In this implementation, the grammar supports an expression "E" which can be either:
//     E := "(" E* ")"  or "a"
// which is parsed using a recursive function without any depth check.
////////////////////////////////////////////////////////////////////////////////////////////////////
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

    // Unsafe helper to read the next character from the string buffer.
    pub unsafe fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.data.len() {
            return None;
        }
        let ptr = self.data.as_ptr().add(self.pos);
        self.pos += 1;
        Some(*ptr as char)
    }

    // Recursively parses an expression from the input.
    // WARNING: There is no recursion depth check in this implementation.
    pub fn parse_expr(&mut self) -> Result<Node, String> {
        let ch = unsafe { self.next_char() }.ok_or("Unexpected end of input")?;
        if ch == '(' {
            let mut children = Vec::new();
            loop {
                // Peek the next character unsafely.
                if self.pos < self.data.len() {
                    let peek = unsafe { *self.data.as_ptr().add(self.pos) as char };
                    if peek == ')' {
                        // Consume the closing parenthesis.
                        unsafe { self.next_char() };
                        break;
                    }
                } else {
                    return Err("Missing closing parenthesis".into());
                }
                // Recursive call with no depth limit.
                let child = self.parse_expr()?;
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
    parser.parse_expr()
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