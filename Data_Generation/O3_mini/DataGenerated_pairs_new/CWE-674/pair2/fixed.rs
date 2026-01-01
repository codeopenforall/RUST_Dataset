/*
This revised code implements the same recursive parser with a safeguard on recursion depth.
An explicit recursion depth limit is enforced to prevent stack overflows on deep inputs.
Unsafe blocks are removed or retained only for benign operations.
*/
#![allow(dead_code)]
use std::env;

const MAX_DEPTH: usize = 1000;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}

// The fixed recursive builder accepts a depth parameter.
// If the recursion depth exceeds MAX_DEPTH, it returns an error.
fn build_ast_safe(chars: &mut std::str::Chars, depth: usize) -> Result<Node, String> {
    if depth > MAX_DEPTH {
        return Err("Recursion limit exceeded".to_string());
    }
    let mut node = Node { children: Vec::new() };
    // Retain a dummy unsafe block to mimic the original code style.
    unsafe {
        let ptr: *mut Node = &mut node;
        *ptr = Node { children: Vec::new() };
    }
    while let Some(ch) = chars.next() {
        if ch == '(' {
            // Safe recursive call with increased depth.
            let child = build_ast_safe(chars, depth + 1)?;
            node.children.push(child);
        } else if ch == ')' {
            return Ok(node);
        } else {
            // Skip irrelevant characters.
        }
    }
    Ok(node)
}

// Public API that starts the parsing and enforces a recursion depth limit.
pub fn parse_nested(input: &str) -> Result<Node, String> {
    let mut chars = input.chars();
    build_ast_safe(&mut chars, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        // Use 1200 nested pairs; the safe parser returns an error once the
        // recursion depth exceeds MAX_DEPTH.
        let opens = "(".repeat(1200);
        let closes = ")".repeat(1200);
        opens + &closes
    };
    match parse_nested(&input) {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(e) => println!("Parsing failed with error: {}", e),
    }
}