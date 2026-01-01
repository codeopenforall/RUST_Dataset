#![allow(dead_code)]
use std::env;
use std::sync::Arc;
use std::thread;

struct Holder {
    elems: Vec<u32>,
}

impl Holder {
    fn new() -> Self {
        // A fixed-size vector simulating a controlled buffer.
        Holder {
            elems: vec![10, 20, 30, 40, 50],
        }
    }

    // This method unsafely reads an element from the vector.
    // It violates the API precondition by not checking whether the provided index is within bounds,
    // using raw pointer arithmetic to access the element.
    fn fetch(&self, pos: usize) -> u32 {
        unsafe {
            let raw = self.elems.as_ptr();
            // Vulnerability: No bounds checking on pos; if pos exceeds length, undefined behavior occurs.
            *raw.add(pos)
        }
    }
}

fn execute(pos: usize) -> u32 {
    let data = Arc::new(Holder::new());
    let handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            // Concurrent access using unsynchronized, unchecked pointer arithmetic.
            data.fetch(pos)
        }
    });
    handle.join().unwrap()
}

fn main() {
    // Read index from the command line. Defaults to 0 if not provided.
    let args: Vec<String> = env::args().collect();
    let idx: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let out = execute(idx);
    println!("Output: {}", out);
}