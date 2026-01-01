//////////////////////////////////////////////////////////////
// Vulnerability example: Uncontrolled recursion can lead to
// a stack overflow when processing deeply nested inputs.
// This example uses unsafe pointer arithmetic and recursion
// without any depth limits. An attacker can create input with
// excessive nesting (e.g. a tree with high depth) that triggers
// a stack overflow.
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    // A tree node that can have many children.
    children: Vec<Data>,
}

impl Data {
    fn new() -> Self {
        Data { children: Vec::new() }
    }
    fn attach(&mut self, child: Data) {
        self.children.push(child);
    }
}

unsafe fn explore(inner: &Data) -> u32 {
    // Uncontrolled recursive function.
    let mut count: u32 = 1; 
    let raw_ptr = inner.children.as_ptr();
    for i in 0..inner.children.len() {
        // Using unsafe raw pointer arithmetic instead of safe iterator.
        let child = &*raw_ptr.add(i);
        count += explore(child);
    }
    count
}

fn collect(root: &Data) -> Result<u32, &'static str> {
    // No safeguard on recursion depth.
    unsafe { Ok(explore(root)) }
}

// Helper function to build a deeply nested tree.
fn construct(depth: usize) -> Data {
    let mut node = Data::new();
    if depth > 0 {
        node.attach(construct(depth - 1));
    }
    node
}

fn main() {
    // Concurrency is simulated via thread spawning.
    let handle = thread::spawn(|| {
        // A tree with very deep nesting, likely to trigger a stack overflow.
        let tree = construct(2000);
        match collect(&tree) {
            Ok(sum) => println!("Total count: {}", sum),
            Err(err) => println!("Error: {}", err),
        }
    });
    // Wait for the thread to complete.
    handle.join().unwrap();
}