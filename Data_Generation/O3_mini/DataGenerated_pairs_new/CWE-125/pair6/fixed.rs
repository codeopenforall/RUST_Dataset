///////////////////////////////////////////////
// Corrected Rust Implementation (Safe Path)
///////////////////////////////////////////////
use std::env;
use std::process;

trait AccessItem {
    fn retrieve(&self, idx: usize) -> Result<u32, &'static str>;
}

struct Buffer {
    data: &'static [u32],
}

impl AccessItem for Buffer {
    fn retrieve(&self, idx: usize) -> Result<u32, &'static str> {
        // FIX: Ensure the index is within the bounds of the valid slice.
        if idx < self.data.len() {
            Ok(self.data[idx])
        } else {
            Err("index out of bounds")
        }
    }
}

fn create_buffer() -> Buffer {
    // The full static array has 4 elements but we expose only the first 3 as valid.
    static FULL: [u32; 4] = [10, 20, 30, 0];
    let slice = &FULL[..3];
    Buffer { data: slice }
}

fn run(idx: usize) -> Result<u32, &'static str> {
    let buffer = create_buffer();
    buffer.retrieve(idx)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <index>");
        process::exit(1);
    }
    let idx: usize = args[1].parse().unwrap_or(0);
    // Attempts to read out-of-bounds are properly checked.
    match run(idx) {
        Ok(val) => println!("Value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}