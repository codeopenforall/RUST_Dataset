use std::sync::{Arc, Mutex};
use std::thread;
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
unsafe fn explore(inner: &Data) -> u32 {
    let mut count: u32 = 1; 
    let raw_ptr = inner.children.as_ptr();
    for i in 0..inner.children.len() {
        let child = &*raw_ptr.add(i);
        count += explore(child);
    }
    count
}
fn collect(root: &Data) -> Result<u32, &'static str> {
    unsafe { Ok(explore(root)) }
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
