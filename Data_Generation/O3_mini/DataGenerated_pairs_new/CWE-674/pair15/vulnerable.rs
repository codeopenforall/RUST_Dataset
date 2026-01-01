//////////////////////////////////////////
// Vulnerability demonstration example //
// CWE-674: Uncontrolled Recursion       //
//////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum Ast {
    Node(Box<Ast>),
    Leaf(char),
}

struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    // Recursive descent without depth limits.
    fn parse_expr(&mut self) -> Result<Ast, String> {
        if self.pos >= self.input.len() {
            return Err("unexpected end".into());
        }
        // Using an unsafe block to perform unchecked pointer arithmetic.
        let c = unsafe { *self.input.as_ptr().add(self.pos) };
        self.pos += 1;
        if c == '(' {
            // Recursive call without checking depth (vulnerable to stack overflow).
            let node = self.parse_expr()?; // <-- Vulnerability: Unbounded recursive call
            // Expect a closing ')'
            if self.pos >= self.input.len() {
                return Err("expected ')' but found end".into());
            }
            let c2 = unsafe { *self.input.as_ptr().add(self.pos) };
            if c2 != ')' {
                return Err("expected ')'".into());
            }
            self.pos += 1;
            Ok(Ast::Node(Box::new(node)))
        } else {
            Ok(Ast::Leaf(c))
        }
    }
}

fn execute(input: &str) -> Result<Ast, String> {
    let mut parser = Parser::new(input);
    parser.parse_expr()
}

fn main() {
    // Construct a deeply nested input to trigger uncontrolled recursion.
    // Example: "((" + 2000 times "(" + "x" + 2000 times ")" + ")"
    let input = "((".to_owned() + &"(".repeat(2000) + "x" + &")".repeat(2000) + ")";
    // Use Arc and Mutex to simulate concurrent access.
    let result_arc = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result_arc);
    let input_clone = input.clone();
    let handle = thread::spawn(move || {
        let res = execute(&input_clone);
        let mut guard = result_clone.lock().unwrap();
        *guard = Some(res);
    });
    handle.join().unwrap();
    let guard = result_arc.lock().unwrap();
    match &*guard {
        Some(Ok(ast)) => println!("Parsed AST: {:?}", ast),
        Some(Err(e)) => println!("Error during parsing: {}", e),
        None => println!("No result produced"),
    }
}