/*
This program incorporates a fix to prevent out-of-bound memory reads.
It uses safe access methods to validate buffer indices before accessing memory,
thus mitigating the possibility of a CWE-125 vulnerability.
*/
use std::env;

struct DataStore {
    buffer: Vec<u8>,
}

impl DataStore {
    fn new() -> Self {
        DataStore {
            buffer: vec![10, 20, 30, 40, 50],
        }
    }

    // Safely reads from the buffer using get() with bounds checking.
    // If the supplied index is out-of-range, the function will panic with an error message.
    fn fetch(&self, position: usize) -> u8 {
        self.buffer.get(position).copied().expect("Index out-of-bounds")
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
    let value = store.fetch(pos);
    println!("Fetched value: {}", value);
}