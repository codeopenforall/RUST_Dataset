use std::env;
use std::thread;
const MAX_ALLOCATION: usize = 1_000_000; 
pub struct ResourceManager;
impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager
    }
    pub fn process(&self, alloc: usize) -> Result<u64, &'static str> {
        if alloc > MAX_ALLOCATION {
            return Err("Allocation size exceeds permitted limit");
        }
        let handle = thread::spawn(move || -> u64 {
            let mut buffer: Vec<u8> = Vec::with_capacity(alloc);
            unsafe {
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
