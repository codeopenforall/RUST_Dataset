//////////////////////
// Vulnerable Code  //
//////////////////////
use std::env;
use std::thread;

const DEFAULT_CAP: usize = 100; // Intended maximum allocation threshold (but not enforced)

struct Processor;

impl Processor {
    // This function spawns a thread that unsafely allocates memory based on the request.
    // It does not enforce any limits on the allocation size.
    pub fn handle(&self, req: usize) -> Result<usize, &'static str> {
        // Spawn a thread to simulate concurrent processing.
        let t = thread::spawn(move || {
            // Unsafe block: directly allocate and set the length of the vector without bound checks.
            // This can result in allocation of excessive memory if req is too large.
            unsafe {
                // The vulnerability: no cap on the allocation size, leading to potential resource exhaustion.
                let mut buf: Vec<u8> = Vec::with_capacity(req);
                // NOTE: set_len is unsafe because it tells the vector it has initialized elements.
                buf.set_len(req);
                // Do some dummy computation: sum of the allocated bytes (which are uninitialized).
                // For deterministic behavior, we override the memory to 1 thus ensuring defined behavior.
                for i in 0..req {
                    // Direct pointer manipulation to write the value '1'
                    *buf.as_mut_ptr().offset(i as isize) = 1;
                }
                buf.iter().fold(0usize, |acc, &x| acc + (x as usize))
            }
        });
        // Wait for the thread to finish and retrieve the result.
        match t.join() {
            Ok(sum) => Ok(sum),
            Err(_) => Err("Thread panicked"),
        }
    }
}

fn main() {
    // Read allocation request from command line argument, defaulting to 50 if not provided.
    let args: Vec<String> = env::args().collect();
    let request: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(50)
    } else {
        50
    };
    
    let processor = Processor;
    // In the vulnerable code, any request is processed without limit.
    match processor.handle(request) {
        Ok(result) => {
            println!("Result is: {}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}