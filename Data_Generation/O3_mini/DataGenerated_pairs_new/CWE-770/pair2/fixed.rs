///////////////////////////////
// Fixed Code Example
///////////////////////////////
use std::env;
use std::thread;

// Define a maximum permitted number of threads.
const MAX_ALLOWED: usize = 10;

pub struct Controller;

impl Controller {
    // Now the method first checks if the requested thread count exceeds a predetermined limit.
    pub fn process(&self, count: usize) -> Result<(), String> {
        if count > MAX_ALLOWED {
            return Err(format!("Requested thread count {} exceeds the limit of {}", count, MAX_ALLOWED));
        }
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            let handle = thread::spawn(|| {
                unsafe {
                    let _data = allocate_buffer(1000);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "A thread panicked".to_string())?;
        }
        Ok(())
    }
}

unsafe fn allocate_buffer(size: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);
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
    // Obtain the thread count from the command-line argument.
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    // Process with appropriate throttling.
    match run_alloc(count) {
        Ok(_) => println!("Processing Completed"),
        Err(e) => eprintln!("Processing Error: {}", e),
    }
}