/*
This example demonstrates a resource allocation routine that fails to throttle input.
It spawns a thread that unsafely writes to a fixed‐size buffer (50 elements)
with a loop bounded by an unchecked input value. When an input greater than 50 is provided,
the unsafe block writes out-of-bounds, leading to potential memory corruption.

Compile with: cargo build --bin vulnerable_app
Run with: ./vulnerable_app <number>
*/
use std::env;
use std::thread;

struct ResourceManager;

impl ResourceManager {
    // This method unsafely writes to a fixed-size vector.
    // If the input count is greater than the buffer capacity (50),
    // the unsafe pointer arithmetic will write past the allocation.
    fn execute(&self, count: usize) -> Result<u32, &'static str> {
        // Fixed-size buffer with 50 u32 elements (initialized to zero)
        let mut buffer = vec![0u32; 50];
        // Spawn a thread to simulate concurrent resource handling.
        let handler = thread::spawn(move || {
            // Vulnerability: no check to prevent count > 50.
            for i in 0..count {
                unsafe {
                    // Unsafe block writing into buffer without bounds check.
                    // Potential out-of-bound write if count > buffer.len() (i.e., 50).
                    let ptr = buffer.as_mut_ptr();
                    *ptr.add(i) = 42;
                }
            }
            // Return the sum of the buffer.
            buffer.iter().sum()
        });
        // Join the thread and propagate any panics as an error.
        handler.join().map_err(|_| "Thread execution error")
    }
}

fn main() {
    // Read input from the command line; expects one argument representing count.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <count>", args[0]);
        std::process::exit(1);
    }
    let count: usize = args[1].parse().unwrap_or(0);
    let manager = ResourceManager;
    match manager.execute(count) {
        Ok(sum) => println!("Operation result: {}", sum),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}