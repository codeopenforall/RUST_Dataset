use std::sync::{Arc, Mutex};
use std::thread;

struct Tree {
    branches: Vec<Tree>,
    token: Option<char>,
}

impl Tree {
    // UNSAFE recursive function without any depth guarding.
    // CWE-674: Uncontrolled Recursion – may lead to stack overflow on overly nested input.
    unsafe fn accumulate(&self) -> usize {
        let mut total = 1;
        // Unsafe pointer arithmetic simulation (for demonstration)
        let _dummy_ptr: *const Tree = self as *const _;
        for child in &self.branches {
            // Recursive call without depth bounds.
            total += child.accumulate();
        }
        total
    }
}

fn build_tree(input: &str) -> Tree {
    let bytes = input.as_bytes();
    let (node, _) = read_node(bytes, 0);
    node
}

fn read_node(data: &[u8], pos: usize) -> (Tree, usize) {
    if pos >= data.len() {
        return (Tree { branches: Vec::new(), token: None }, pos);
    }
    if data[pos] == b'(' {
        let mut kids = Vec::new();
        let mut idx = pos + 1;
        while idx < data.len() && data[idx] != b')' {
            let (child, new_idx) = read_node(data, idx);
            kids.push(child);
            idx = new_idx;
        }
        (Tree { branches: kids, token: None }, idx + 1)
    } else {
        // Treat any non-'(' character as a leaf token.
        (Tree { branches: Vec::new(), token: Some(data[pos] as char) }, pos + 1)
    }
}

fn execute(input: &str) -> Result<usize, &'static str> {
    let tree = build_tree(input);
    let shared = Arc::new(Mutex::new(tree));
    let shared_copy = Arc::clone(&shared);
    let worker = thread::spawn(move || {
        let lock = shared_copy.lock().unwrap();
        // If the tree is extremely deep, this recursive call might overflow the stack.
        unsafe { lock.accumulate() }
    });
    // Returning Ok(..) if thread completes. A stack overflow panic will abort execution.
    Ok(worker.join().unwrap())
}

fn main() {
    let sample = "(a(a(a(a(a))))))"; // Example input; not deeply nested.
    match execute(sample) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => eprintln!("Error: {}", err),
    }
}