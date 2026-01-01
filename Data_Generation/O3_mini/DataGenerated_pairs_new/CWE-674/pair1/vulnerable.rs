//////////////////////////////////////////////
// Vulnerable Example for Uncontrolled Recursion
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { value: val, children: Vec::new() }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }
}

/// Recursively traverses the tree without checking recursion limits.
/// Uses unsafe pointer arithmetic to iterate over children.
pub unsafe fn traverse_recursive(node: &Node) -> i32 {
    let mut total = node.value;
    // Unsafe pointer arithmetic: no bounds check on recursion depth.
    let ptr = node.children.as_ptr();
    for i in 0..node.children.len() {
        let child_ptr = ptr.add(i);
        // Vulnerable recursive call without depth limit - CWE-674.
        total += traverse_recursive(&**child_ptr);
    }
    total
}

/// Processes the tree and returns its cumulative value.
/// Returns Some(total) upon success. With deeply nested inputs this may trigger stack overflow.
pub fn process_tree(root: &Node) -> Option<i32> {
    unsafe { Some(traverse_recursive(root)) }
}

fn main() {
    // Construct a deeply nested tree (chain) to demonstrate uncontrolled recursion.
    let mut root = Node::new(1);
    let mut current = &mut root;
    for i in 2..=1500 {
        current.add_child(Node::new(i));
        let last_index = current.children.len() - 1;
        current = current.children[last_index].as_mut();
    }

    // Spawn a thread to process the tree.
    let handle = thread::spawn(move || {
        let result = process_tree(&root);
        if let Some(sum) = result {
            println!("Sum: {}", sum);
        }
        result
    });

    match handle.join() {
        Ok(Some(sum)) => println!("Final Sum: {}", sum),
        Ok(None) => println!("Completed without a result"),
        Err(_) => println!("Thread panicked due to recursion depth"),
    }
}