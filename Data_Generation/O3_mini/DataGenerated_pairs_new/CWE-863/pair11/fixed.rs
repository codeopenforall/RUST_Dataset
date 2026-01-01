////////////////////
// Fixed Code     //
////////////////////
use std::env;
use std::thread;

const MAX_ALLOCATION: usize = 100; // Define a maximum allowed allocation threshold

struct Processor;

impl Processor {
    // This function spawns a thread to handle the allocation request but enforces a maximum cap.
    pub fn handle(&self, req: usize) -> Result<usize, &'static str> {
        // Enforce allocation limit: Reject requests exceeding MAX_ALLOCATION.
        if req > MAX_ALLOCATION {
            return Err("Allocation request exceeds allowed threshold");
        }
        let t = thread::spawn(move || {
            unsafe {
                let mut buf: Vec<u8> = Vec::with_capacity(req);
                buf.set_len(req);
                // Initialize allocated memory safely by writing defined value.
                for i in 0..req {
                    *buf.as_mut_ptr().offset(i as isize) = 1;
                }
                buf.iter().fold(0usize, |acc, &x| acc + (x as usize))
            }
        });
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