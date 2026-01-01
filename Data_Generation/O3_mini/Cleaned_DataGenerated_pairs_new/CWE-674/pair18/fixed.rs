#![allow(unused_unsafe)]
use std::env;
const MAX_DEPTH: u32 = 1000;
fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    unsafe { safe_parse_helper(bytes, 0, 0) }?;
    Ok(())
}
unsafe fn safe_parse_helper(data: &[u8], pos: usize, depth: u32) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("maximum recursion depth exceeded");
    }
    let mut i = pos;
    while i < data.len() {
        let ch = *data.get_unchecked(i);
        if ch == b'(' {
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
        "(".repeat(3500) + &")".repeat(3500)
    };
    match process_input(&deep_input) {
        Ok(_) => println!("Parsing completed successfully."),
        Err(e) => println!("Error encountered: {}", e),
    }
}
