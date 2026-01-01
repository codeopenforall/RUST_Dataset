/*
This program demonstrates a vulnerability by performing an unchecked read from a buffer.
It uses unsafe pointer arithmetic to retrieve an element without bounds checking,
which can lead to out-of-bounds reads (CWE-125) when an invalid index is provided.
*/
use std::env;

struct DataStore {
    buffer: Vec<u8>,
}

impl DataStore {
    fn new() -> Self {
        // A fixed-size buffer with 5 elements.
        DataStore {
            buffer: vec![10, 20, 30, 40, 50],
        }
    }

    // Performs an unchecked read using unsafe block.
    // If `position` is not within the bounds [0, buffer.len()-1],
    // this function leads to undefined behavior.
    fn fetch(&self, position: usize) -> u8 {
        // Vulnerable: No bounds-check, directly using get_unchecked.
        // CWE-125: Out-of-Bounds Read.
        unsafe { *self.buffer.get_unchecked(position) }
    }
}

fn main() {
    let store = DataStore::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <index>", args[0]);
        return;
    }
    let pos: usize = args[1].parse().unwrap_or(0);
    // This call may lead to an unsafe out-of-bounds memory access.
    let value = store.fetch(pos);
    println!("Fetched value: {}", value);
}