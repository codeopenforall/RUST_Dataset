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

    // This method safely reads an element and enforces a bounds check before accessing the element.
    fn fetch(&self, pos: usize) -> u32 {
        if pos < self.elems.len() {
            self.elems[pos]
        } else {
            // Enforce API preconditions by panicking when an invalid index is provided.
            panic!("Provided index {} is out-of-bound (max index {}).", pos, self.elems.len() - 1);
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