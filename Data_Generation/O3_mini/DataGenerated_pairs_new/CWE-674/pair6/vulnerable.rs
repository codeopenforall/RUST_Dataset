//////////////////////////////////////////
// Vulnerability Example (Uncontrolled Recursion)
// CWE-674: Uncontrolled Recursion (Stack Overflow)
// This code implements a recursive parser for nested parentheses 
// using unsafe block for pointer access. It does not enforce any 
// maximum recursion depth, so deeply nested input may result in a 
// stack overflow at runtime.
//////////////////////////////////////////
use std::env;

struct Parser<'a> {
    buffer: &'a [u8],
}

impl<'a> Parser<'a> {
    // Unsafe: directly accessing the internal buffer without bounds check.
    unsafe fn get_byte(&self, index: usize) -> u8 {
        *self.buffer.get_unchecked(index)
    }

    // Recursively parse the input starting from position 'pos'.
    // No recursion depth limit is enforced.
    fn parse_recursively(&self, pos: usize) -> Result<(usize, u32), &'static str> {
        let mut index = pos;
        let mut count: u32 = 0;
        while index < self.buffer.len() {
            unsafe {
                match self.get_byte(index) {
                    b'(' => {
                        // Recursively process nested expression.
                        let (next_index, inner_count) = self.parse_recursively(index + 1)?;
                        count = count.wrapping_add(inner_count).wrapping_add(1);
                        index = next_index;
                    }
                    b')' => {
                        // End of current nested expression.
                        return Ok((index + 1, count));
                    }
                    _ => {
                        index += 1;
                    }
                }
            }
        }
        Ok((index, count))
    }

    // Public interface to start parsing.
    fn run(&self) -> Result<u32, &'static str> {
        let (_, result) = self.parse_recursively(0)?;
        Ok(result)
    }
}

fn process(input: &str) -> Result<u32, &'static str> {
    let parser = Parser { buffer: input.as_bytes() };
    parser.run()
}

fn main() {
    // Construct a deeply nested input to trigger uncontrolled recursion.
    // For example, 10000 nested pairs.
    let left_parens = "(".repeat(10000);
    let right_parens = ")".repeat(10000);
    let input = left_parens + &right_parens;
    match process(&input) {
        Ok(result) => println!("Parsed count: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}