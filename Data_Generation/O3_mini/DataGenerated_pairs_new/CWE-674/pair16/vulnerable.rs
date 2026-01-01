////////////////////////////////////////////
// Vulnerability: Uncontrolled Recursion
// CWE-674 example – a recursive parser for nested parentheses
// without a depth limit. Deeply nested input may cause a stack overflow.
// Uses unsafe pointer arithmetic to mimic realistic low‐level API usage.
////////////////////////////////////////////

use std::env;
use std::ptr;

struct Processor;

impl Processor {
    // Recursive function: traverses the input string via a raw pointer.
    // Vulnerability: no limit to recursion depth.
    unsafe fn traverse(&self, data: *const u8, len: usize) -> Result<usize, String> {
        // Base case: no more data.
        if len == 0 {
            return Ok(0);
        }
        // Read the first byte.
        let first = ptr::read(data);
        if first == b'(' {
            // Recursively process the remainder of the string.
            // Vulnerable recursive call (see line numbers below).
            let inner_depth = self.traverse(data.add(1), len - 1)?;
            // Attempt to find the matching closing parenthesis.
            // Calculate position of corresponding closing symbol.
            if len >= inner_depth + 2 && ptr::read(data.add(inner_depth + 1)) == b')' {
                Ok(inner_depth + 1)
            } else {
                Err("Missing closing parenthesis".to_string())
            }
        } else {
            Err("Unexpected token encountered".to_string())
        }
    }

    pub fn process(&self, input: &str) -> Result<usize, String> {
        unsafe { self.traverse(input.as_ptr(), input.len()) }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Use command-line input if provided; otherwise, a simple default.
    let input = if args.len() > 1 { &args[1] } else { "(())" };
    let proc = Processor;
    match proc.process(input) {
        Ok(depth) => println!("Computed depth: {}", depth),
        Err(e) => eprintln!("Error: {}", e),
    }
}