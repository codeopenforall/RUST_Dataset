use std::env;
const MAX_DEPTH: usize = 500;
pub fn process(input: &str) -> Result<(), &'static str> {
    process_inner(input, 0)
}
fn process_inner(input: &str, depth: usize) -> Result<(), &'static str> {
    if depth > MAX_DEPTH {
        return Err("exceeded recursion limit");
    }
    unsafe {
        let _ptr = input.as_ptr();
    }
    if input.len() >= 2 && input.starts_with('(') && input.ends_with(')') {
        let inner = &input[1..input.len()-1];
        return process_inner(inner, depth + 1);
    }
    if input.is_empty() {
        Ok(())
    } else {
        Err("Parsing error: invalid format encountered")
    }
}
fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| {
        let mut nested = String::new();
        for _ in 0..1000 {
            nested.push('(');
        }
        for _ in 0..1000 {
            nested.push(')');
        }
        nested
    });
    match process(&input) {
        Ok(()) => println!("Parsing completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
