//////////////////////////////////////////////////////////////
// This Rust program intentionally demonstrates an out‐of‐bounds
// read vulnerability (CWE-125) by using an unsafe block to access
// a vector without proper bounds checking in a concurrent context.
// It defines a structure to hold data and spawns a thread that
// calls an unchecked read on an index clearly beyond the buffer
// length. Note that the code uses unsafe constructs and Arc to
// simulate realistic concurrency issues witnessed in real-world
// vulnerability reports.
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

    // This method performs an unchecked read.
    // CWE-125: Out-of-Bounds Read - No bounds checking is performed.
    pub fn fetch(&self, idx: usize) -> u8 {
        unsafe { *self.data.get_unchecked(idx) }
    }
}

// Executes the read operation in a new thread.
pub fn execute() -> u8 {
    let holder = Arc::new(DataHolder::new());
    let shared = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        // Intentionally using an out-of-bound index (10) on a 5-element array.
        shared.fetch(10)
    });
    handle.join().unwrap()
}

fn main() {
    // This call is expected to trigger undefined behavior (likely a panic)
    // because it performs an out-of-bound read.
    let res = execute();
    println!("Result: {}", res);
}