#![allow(dead_code)]
use std::env;
#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}
impl Node {
    unsafe fn alloc() -> *mut Node {
        let node = Box::new(Node { children: Vec::new() });
        Box::into_raw(node)
    }
    unsafe fn dealloc(ptr: *mut Node) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    }
}
fn build_ast(chars: &mut std::str::Chars) -> Node {
    let mut node = Node { children: Vec::new() };
    unsafe {
        let ptr: *mut Node = &mut node;
        *ptr = Node { children: Vec::new() };
    }
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let child = build_ast(chars);
            node.children.push(child);
        } else if ch == ')' {
            return node;
        } else {
        }
    }
    node
}
pub fn parse_nested(input: &str) -> Result<Node, ()> {
    let mut chars = input.chars();
    let ast = build_ast(&mut chars);
    Ok(ast)
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
        Err(_) => println!("Parsing failed."),
    }
}
