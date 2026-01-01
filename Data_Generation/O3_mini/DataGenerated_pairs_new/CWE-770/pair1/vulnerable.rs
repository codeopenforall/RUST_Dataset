//////////////////////////////////////////////
// Vulnerability Example Code (Resource Exhaustion)
// CWE-770: Allocation of Resources Without Limits or Throttling
//////////////////////////////////////////////

use std::env;
use std::thread;

pub struct ResourceManager;

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager
    }

    // The process routine allocates a vector of user‐provided size
    // without imposing any upper limit.
    // It uses an unsafe block to set the vector length directly,
    // which can lead to undefined behavior if the allocation size is huge.
    pub fn process(&self, alloc: usize) -> Result<u64, &'static str> {
        let handle = thread::spawn(move || -> u64 {
            // Potential resource exhaustion:
            // No cap on the allocation size, and unsafe set_len is used.
            let mut buffer: Vec<u8> = Vec::with_capacity(alloc);
            unsafe {
                // WARNING: Using set_len without ensuring the allocated memory is fully valid.
                buffer.set_len(alloc);
            }
            // Initialize the buffer; if 'alloc' is huge, this loop can consume enormous CPU and memory.
            for i in 0..alloc {
                // Deliberately writing without bounds check elsewhere.
                buffer[i] = 42;
            }
            // Return the sum of the buffer values.
            buffer.iter().map(|&b| b as u64).sum()
        });
        // Wait for the thread to complete.
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