//////////////////////////
// Vulnerable Code Sample
//////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct BufferManager {
    data: Vec<u8>,
}

impl BufferManager {
    // UNSAFE: Converts a portion of the byte buffer to &str without boundary checking.
    // This function improperly trusts input parameters (start and length),
    // which may lead to out-of-bounds memory access.
    unsafe fn get_segment(&self, start: usize, length: usize) -> &str {
        // Vulnerability: No check that start + length is within self.data.len()
        let ptr = self.data.as_ptr().add(start);
        let slice = std::slice::from_raw_parts(ptr, length);
        std::str::from_utf8_unchecked(slice)
    }
}

struct Processor {
    manager: Arc<Mutex<BufferManager>>,
}

impl Processor {
    // Processes a numeric value from a substring extracted unsafely.
    fn run(&self, start: usize, length: usize) -> Result<u32, &'static str> {
        let guard = self.manager.lock().unwrap();
        // Unsafe conversion without proper input validation.
        let segment = unsafe { guard.get_segment(start, length) };
        segment.trim().parse::<u32>().map_err(|_| "parse error")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: <program> <input_string> <start> <length>");
        return;
    }
    let input = args[1].clone();
    let start: usize = args[2].parse().unwrap_or(0);
    let length: usize = args[3].parse().unwrap_or(0);
    let manager = BufferManager { data: input.into_bytes() };
    let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };

    let handle = thread::spawn(move || {
        match proc_inst.run(start, length) {
            Ok(value) => println!("Extracted number: {}", value),
            Err(err) => println!("Operation failed: {}", err),
        }
    });
    handle.join().unwrap();
}