/* 
   This Rust program now implements a recursive parser that safely walks through a series of nested parentheses.
   It uses a recursion depth counter to prevent uncontrolled recursion.
   If the recursion depth exceeds MAX_DEPTH, the function returns an error.
   Unsafe pointer access is still used for performance, but the depth check ensures attacker-supplied inputs 
   will be handled gracefully rather than crashing (CWE-674 mitigated).
*/
#![allow(unused_unsafe)]
use std::env;

const MAX_DEPTH: u32 = 1000;

fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    unsafe { safe_parse_helper(bytes, 0, 0) }?;
    Ok(())
}

// Recursive helper function that tracks the current recursion depth.
// Returns an error if MAX_DEPTH is exceeded.
unsafe fn safe_parse_helper(data: &[u8], pos: usize, depth: u32) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("maximum recursion depth exceeded");
    }
    let mut i = pos;
    while i < data.len() {
        let ch = *data.get_unchecked(i);
        if ch == b'(' {
            // Increment recursion depth on recursive call.
            i = safe_parse_helper(data, i + 1, depth + 1)?;
        } else if ch == b')' {
            return Ok(i + 1);
        }
        i += 1;
    }
    Ok(i)
}

fn main() {
    let deep_input = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        // Use a deeply nested input that would normally trigger uncontrolled recursion.
        "(".repeat(3500) + &")".repeat(3500)
    };

    match process_input(&deep_input) {
        Ok(_) => println!("Parsing completed successfully."),
        Err(e) => println!("Error encountered: {}", e),
    }
}