/////////////////////// Vulnerable Code ///////////////////////
// This code implements a recursive parser for nested parentheses without
// imposing any maximum recursion depth. It uses unsafe pointer arithmetic to advance
// through the input. Deeply nested inputs may lead to stack overflows due to
// uncontrolled recursion.
//
// CWE-674: Uncontrolled Recursion
//
// To run, compile with: rustc vulnerable.rs
use std::ptr;

struct Parser {
    data: *const u8,
    len: usize,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        // SAFETY: We immediately use input as a byte slice.
        let bytes = input.as_bytes();
        Parser {
            data: bytes.as_ptr(),
            len: bytes.len(),
            pos: 0,
        }
    }

    // Advance by one character unsafely.
    fn next_byte(&mut self) -> Option<u8> {
        if self.pos < self.len {
            // Unsafe pointer arithmetic mimicking a common pattern in low-level parsers.
            let byte = unsafe { ptr::read(self.data.add(self.pos)) };
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    // Recursively parse nested parentheses.
    // For each '(' encountered, the function calls itself without any depth-check.
    fn parse(&mut self) -> Result<(), &'static str> {
        match self.next_byte() {
            Some(b'(') => {
                // Begin a new nested recursion.
                self.parse()?; // Recursion without any depth guard.
                // Expect a matching ')'.
                match self.next_byte() {
                    Some(b')') => Ok(()),
                    _ => Err("Missing closing parenthesis"),
                }
            },
            Some(b')') => Err("Unexpected closing parenthesis"),
            Some(_) => self.parse(), // Skip other characters.
            None => Ok(()),
        }
    }
}

fn process(input: &str) -> Result<(), &'static str> {
    let mut p = Parser::new(input);
    p.parse()
}

fn main() {
    // Example input: a shallow nested expression.
    let input = "(())";
    match process(input) {
        Ok(_) => println!("Parsed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
/////////////////////// End Vulnerable Code ///////////////////////