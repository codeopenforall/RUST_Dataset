/*
This code implements a recursive parser that builds an AST from a nested parenthesis expression.
It uses unsafe blocks to manipulate raw pointers in a real‐world style.
The recursion is uncontrolled so that extremely deep inputs cause a stack overflow.
*/
#![allow(dead_code)]
use std::env;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}

impl Node {
    // An unsafe constructor that creates a Node from a Box pointer.
    unsafe fn alloc() -> *mut Node {
        let node = Box::new(Node { children: Vec::new() });
        Box::into_raw(node)
    }

    // An unsafe deallocation function.
    unsafe fn dealloc(ptr: *mut Node) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    }
}

// The recursive helper function that builds the tree without any recursion limit.
// Unsafe block used for a dummy raw pointer manipulation.
fn build_ast(chars: &mut std::str::Chars) -> Node {
    let mut node = Node { children: Vec::new() };
    // Dummy unsafe operation: rewriting the node via raw pointer.
    unsafe {
        let ptr: *mut Node = &mut node;
        // Overwrite the node with an empty one; real code might do low-level memory work.
        *ptr = Node { children: Vec::new() };
    }
    while let Some(ch) = chars.next() {
        if ch == '(' {
            // Uncontrolled recursion -- vulnerability: no maximum depth check!
            let child = build_ast(chars);
            node.children.push(child);
        } else if ch == ')' {
            return node;
        } else {
            // Ignore any non-parenthesis, whitespace etc.
        }
    }
    node
}

// Public API that starts parsing. Deep recursion may cause stack overflow.
pub fn parse_nested(input: &str) -> Result<Node, ()> {
    let mut chars = input.chars();
    let ast = build_ast(&mut chars);
    Ok(ast)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // If an argument is given, use it; otherwise, use a preset deep nested input.
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        // 1200 nested pairs which will likely overflow the stack.
        let opens = "(".repeat(1200);
        let closes = ")".repeat(1200);
        opens + &closes
    };
    // This call may trigger a stack overflow if the input is too deep.
    match parse_nested(&input) {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(_) => println!("Parsing failed."),
    }
}