//////////////////////////////////////////////
// Corrected Example Code (Resource Exhaustion Mitigation)
// Limits the allocation size to a defined maximum.
// CWE-770 Mitigation: Imposing resource usage limits.
//////////////////////////////////////////////

use std::env;
use std::thread;

const MAX_ALLOCATION: usize = 1_000_000; // maximum allowed allocation size

pub struct ResourceManager;

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager
    }

    // The process routine now validates the allocation size against a cap.
    // If the size exceeds the maximum allowed, it returns an error.
    pub fn process(&self, alloc: usize) -> Result<u64, &'static str> {
        if alloc > MAX_ALLOCATION {
            return Err("Allocation size exceeds permitted limit");
        }
        let handle = thread::spawn(move || -> u64 {
            let mut buffer: Vec<u8> = Vec::with_capacity(alloc);
            unsafe {
                // Safe because we've now limited 'alloc' to a known, acceptable range.
                buffer.set_len(alloc);
            }
            for i in 0..alloc {
                buffer[i] = 42;
            }
            buffer.iter().map(|&b| b as u64).sum()
        });
        handle.join().map_err(|_| "Thread panicked")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: <binary> <allocation_size>");
        return;
    }
    let alloc: usize = args[1].parse().unwrap_or(0);
    let mgr = ResourceManager::new();
    match mgr.process(alloc) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(err) => println!("Error: {}", err),
    }
}