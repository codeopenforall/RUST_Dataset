//////////////////////////////////////////////
// Fixed Example: Recursion with Depth Limiting
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_DEPTH: u32 = 1000;

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

/// Recursively traverses the tree while enforcing a maximum recursion depth.
/// Uses unsafe pointer arithmetic similar to the vulnerable version, but stops if the depth limit is exceeded.
pub unsafe fn traverse_limited(node: &Node, depth: u32, max_depth: u32) -> Option<i32> {
    if depth > max_depth {
        return None;
    }
    let mut total = node.value;
    let ptr = node.children.as_ptr();
    for i in 0..node.children.len() {
        let child_ptr = ptr.add(i);
        match traverse_limited(&**child_ptr, depth + 1, max_depth) {
            Some(child_sum) => total += child_sum,
            None => return None, // Abort if any branch exceeds the depth.
        }
    }
    Some(total)
}

/// Processes the tree with a depth limit to safely abort on excessively deep recursion.
pub fn process_tree(root: &Node) -> Option<i32> {
    unsafe { traverse_limited(root, 0, MAX_DEPTH) }
}

fn main() {
    // Construct a deeply nested tree (chain) where the depth exceeds the safe limit.
    let mut root = Node::new(1);
    let mut current = &mut root;
    for i in 2..=1500 {
        current.add_child(Node::new(i));
        let last_index = current.children.len() - 1;
        current = current.children[last_index].as_mut();
    }

    // Spawn a thread to process the tree safely.
    let handle = thread::spawn(move || {
        let result = process_tree(&root);
        match result {
            Some(sum) => println!("Sum: {}", sum),
            None => println!("Recursion depth limit reached. Operation aborted."),
        }
        result
    });

    match handle.join() {
        Ok(Some(sum)) => println!("Final Sum: {}", sum),
        Ok(None) => println!("Recursion depth limit reached, operation safely aborted."),
        Err(_) => println!("Thread panicked unexpectedly."),
    }
}