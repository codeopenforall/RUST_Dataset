/* 
   This Rust program implements a recursive parser that walks through a series of nested parentheses.
   It uses unsafe pointer access to mimic a real‐world failure in low-level parsing code.
   The recursion is uncontrolled: no limit is enforced on the recursion depth.
   Under attacker-supplied extreme inputs (overly nested parentheses), the function may cause a
   stack overflow (CWE-674).
*/
#![allow(unused_unsafe)]
use std::env;

fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    // Entering unsafe block to use unchecked indexing.
    unsafe { parse_helper(bytes, 0) }?;
    Ok(())
}

// Recursive helper function that walks the input bytes.
// No depth limit is enforced.
unsafe fn parse_helper(data: &[u8], pos: usize) -> Result<usize, &'static str> {
    let mut i = pos;
    while i < data.len() {
        // Using unsafe unchecked access to mimic performance-oriented code.
        let ch = *data.get_unchecked(i);
        if ch == b'(' {
            // Recursive descent upon encountering an open parenthesis.
            // Vulnerability: No check on recursion depth.
            i = parse_helper(data, i + 1)?;
        } else if ch == b')' {
            return Ok(i + 1);
        }
        i += 1;
    }
    Ok(i)
}

fn main() {
    // For demonstration purposes, if no arguments are provided, use a deep nested string.
    let deep_input = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        // Generate a deeply nested input that can trigger uncontrolled recursion.
        // (3500 pairs should be enough to provoke a stack overflow on many systems.)
        "(".repeat(3500) + &")".repeat(3500)
    };

    match process_input(&deep_input) {
        Ok(_) => println!("Parsing completed successfully."),
        Err(e) => println!("Error encountered: {}", e),
    }
}