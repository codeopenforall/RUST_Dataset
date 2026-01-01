#![allow(dead_code)]
use std::env;
const MAX_DEPTH: usize = 1000;
#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}
fn build_ast_safe(chars: &mut std::str::Chars, depth: usize) -> Result<Node, String> {
    if depth > MAX_DEPTH {
        return Err("Recursion limit exceeded".to_string());
    }
    let mut node = Node { children: Vec::new() };
    unsafe {
        let ptr: *mut Node = &mut node;
        *ptr = Node { children: Vec::new() };
    }
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let child = build_ast_safe(chars, depth + 1)?;
            node.children.push(child);
        } else if ch == ')' {
            return Ok(node);
        } else {
        }
    }
    Ok(node)
}
pub fn parse_nested(input: &str) -> Result<Node, String> {
    let mut chars = input.chars();
    build_ast_safe(&mut chars, 0)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        let opens = "(".repeat(1200);
        let closes = ")".repeat(1200);
        opens + &closes
    };
    match parse_nested(&input) {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(e) => println!("Parsing failed with error: {}", e),
    }
}
