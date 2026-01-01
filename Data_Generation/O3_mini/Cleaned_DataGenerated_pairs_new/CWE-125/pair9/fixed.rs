#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Processor {
    data: Vec<u32>,
}
impl Processor {
    fn new(data: Vec<u32>) -> Self {
        Processor { data }
    }
    fn process_at(&self, index: usize) -> Result<u32, &'static str> {
        if index < self.data.len() {
            unsafe {
                Ok(*self.data.as_ptr().add(index))
            }
        } else {
            Err("Index out of bounds")
        }
    }
}
fn execute(index: usize) -> Result<u32, &'static str> {
    let proc = Processor::new(vec![10, 20, 30, 40]);
    let proc_arc = Arc::new(proc);
    let proc_clone = Arc::clone(&proc_arc);
    let handle = thread::spawn(move || {
        proc_clone.process_at(index)
    });
    handle.join().unwrap()
}
fn main() {
    let idx = 4;
    match execute(idx) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
