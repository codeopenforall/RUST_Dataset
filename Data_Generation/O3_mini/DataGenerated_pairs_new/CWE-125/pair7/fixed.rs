//////////////////////////////////////////////////////////////
// This Rust program fixes the out‐of‐bounds read vulnerability
// by using safe access methods that perform bounds checking.
// The DataHolder structure now employs Vec::get to safely read
// data from the buffer. Additionally, the execute function is
// modified to use a valid index, ensuring the concurrent thread
// reads within the vector bounds.
//////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct DataHolder {
    data: Vec<u8>,
}

impl DataHolder {
    fn new() -> Self {
        // Initialize with a fixed buffer of 5 elements.
        DataHolder { data: vec![1, 2, 3, 4, 5] }
    }

    // Safe read method which checks bounds.
    pub fn fetch(&self, idx: usize) -> u8 {
        self.data.get(idx).copied().expect("Index out of bounds in safe read")
    }
}

// Executes the safe read operation in a new thread.
pub fn execute() -> u8 {
    let holder = Arc::new(DataHolder::new());
    let shared = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        // Use a valid index (2) ensuring safe and deterministic access.
        shared.fetch(2)
    });
    handle.join().unwrap()
}

fn main() {
    // This call now returns a valid result using safe access methods.
    let res = execute();
    println!("Result: {}", res);
}