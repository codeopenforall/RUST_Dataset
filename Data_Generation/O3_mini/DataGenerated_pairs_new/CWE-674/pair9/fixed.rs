//////////////////////////////////////////////////////////////
// Fixed example: Mitigating uncontrolled recursion by 
// introducing a maximum recursion depth. The recursive processing 
// of nested data now checks the depth level and returns an error if it
// exceeds a safe threshold. Safe iteration replaces direct unsafe pointer
// arithmetic.
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const LIMIT: usize = 1000;

struct Data {
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

fn explore_safe(inner: &Data, depth: usize) -> Result<u32, &'static str> {
    if depth > LIMIT {
        return Err("Recursion too deep");
    }
    let mut count: u32 = 1;
    // Safe iteration over children.
    for child in &inner.children {
        count = count.checked_add(explore_safe(child, depth + 1)?) .ok_or("Count overflow")?;
    }
    Ok(count)
}

fn collect(root: &Data) -> Result<u32, &'static str> {
    explore_safe(root, 0)
}

fn construct(depth: usize) -> Data {
    let mut node = Data::new();
    if depth > 0 {
        node.attach(construct(depth - 1));
    }
    node
}

fn main() {
    let handle = thread::spawn(|| {
        let tree = construct(2000);
        match collect(&tree) {
            Ok(sum) => println!("Total count: {}", sum),
            Err(err) => println!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}