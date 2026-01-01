#![allow(dead_code)]
use std::env;
use std::sync::Arc;
use std::thread;
struct Holder {
    elems: Vec<u32>,
}
impl Holder {
    fn new() -> Self {
        Holder {
            elems: vec![10, 20, 30, 40, 50],
        }
    }
    fn fetch(&self, pos: usize) -> u32 {
        unsafe {
            let raw = self.elems.as_ptr();
            *raw.add(pos)
        }
    }
}
fn execute(pos: usize) -> u32 {
    let data = Arc::new(Holder::new());
    let handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            data.fetch(pos)
        }
    });
    handle.join().unwrap()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let idx: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let out = execute(idx);
    println!("Output: {}", out);
}
