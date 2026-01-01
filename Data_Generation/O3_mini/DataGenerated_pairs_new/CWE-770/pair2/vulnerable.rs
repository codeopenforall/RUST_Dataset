///////////////////////////////
// Vulnerable Code Example
///////////////////////////////
use std::env;
use std::thread;

pub struct Controller;

impl Controller {
    // This method spawns as many threads as requested without any verification.
    // Each thread performs an unsafe allocation of a fixed-size buffer.
    pub fn process(&self, count: usize) -> Result<(), String> {
        let mut handles = Vec::with_capacity(count);
        // Vulnerability: No cap is imposed on the number of threads.
        for _ in 0..count {
            // Spawn a thread that unsafely allocates a buffer.
            let handle = thread::spawn(|| {
                unsafe {
                    // Allocate a buffer of 1,000 bytes without validating input.
                    let _data = allocate_buffer(1000);
                }
            });
            handles.push(handle);
        }
        // Wait for all threads to finish.
        for handle in handles {
            handle.join().map_err(|_| "A thread panicked".to_string())?;
        }
        Ok(())
    }
}

unsafe fn allocate_buffer(size: usize) -> Vec<u8> {
    // An unsafe allocation routine that manually fills the vector.
    // There is no check to avoid huge allocation requests.
    let mut buf = Vec::with_capacity(size);
    // Inserting zeros, simulating further use of the allocated resource.
    for _ in 0..size {
        buf.push(0);
    }
    buf
}

pub fn run_alloc(count: usize) -> Result<(), String> {
    let ctl = Controller;
    ctl.process(count)
}

fn main() {
    // Read the desired thread count from the first command-line argument.
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    // Process without any limits.
    match run_alloc(count) {
        Ok(_) => println!("Processing Completed"),
        Err(e) => eprintln!("Processing Error: {}", e),
    }
}