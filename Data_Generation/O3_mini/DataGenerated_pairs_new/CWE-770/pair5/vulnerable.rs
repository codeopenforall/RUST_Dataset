//////////////////////////////////////////
// Vulnerability Example
//////////////////////////////////////////
use std::alloc::{alloc, Layout};
use std::env;
use std::sync::Arc;
use std::thread;

trait Processor {
    fn run(&self, count: usize) -> Result<(), &'static str>;
}

struct ResourceManager;

impl Processor for ResourceManager {
    fn run(&self, count: usize) -> Result<(), &'static str> {
        // Create a layout based solely on user input without any cap
        let layout = Layout::array::<u8>(count).map_err(|_| "Layout error")?;
        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return Err("Allocation failed");
            }
            // Fill the allocated memory unsafely without bounds protection
            for i in 0..count {
                ptr.add(i).write(42);
            }
            // Transform the buffer into a Vec without checking resource limits
            let vec = Vec::from_raw_parts(ptr, count, count);
            // Process the data concurrently in several threads
            let shared = Arc::new(vec);
            let mut workers = vec![];
            for _ in 0..4 {
                let data = Arc::clone(&shared);
                workers.push(thread::spawn(move || {
                    let _sum: u8 = data.iter().fold(0, |acc, &x| acc.wrapping_add(x));
                }));
            }
            for worker in workers {
                worker.join().unwrap();
            }
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(100)
    } else {
        100
    };
    let manager = ResourceManager;
    if let Err(e) = manager.run(count) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}