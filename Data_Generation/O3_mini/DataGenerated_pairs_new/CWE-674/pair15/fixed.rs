//////////////////////////////////////////
// Corrected example with Recursion Limit//
// CWE-674 Fix: Enforcing maximum recursion depth
//////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

const MAX_DEPTH: usize = 1000;

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

    // Recursive descent with an added recursion depth counter.
    fn parse_expr_with_depth(&mut self, depth: usize) -> Result<Ast, String> {
        if depth > MAX_DEPTH {
            return Err("recursion limit exceeded".into());
        }
        if self.pos >= self.input.len() {
            return Err("unexpected end".into());
        }
        // Unsafe block remains to mimic low-level access.
        let c = unsafe { *self.input.as_ptr().add(self.pos) };
        self.pos += 1;
        if c == '(' {
            let node = self.parse_expr_with_depth(depth + 1)?;
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
    parser.parse_expr_with_depth(0)
}

fn main() {
    // Use a less deeply nested input that remains within safe recursion limits.
    // Example: "((" + 500 times "(" + "x" + 500 times ")" + ")"
    let input = "((".to_owned() + &"(".repeat(500) + "x" + &")".repeat(500) + ")";
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