//// Vulnerable Rust Code for Improper Input Validation (CWE-020)
use std::env;
use std::sync::{Arc};
use std::thread;
use std::str;

struct Processor {
    buffer: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        // The buffer contains sensitive data.
        Processor { buffer: b"Sensitive buffer data".to_vec() }
    }

    // Processes the user-supplied length without proper validation.
    fn process(&self, len: usize) -> Result<String, &'static str> {
        // Vulnerability: Using unsafe conversion without checking that len is within the buffer bounds.
        unsafe {
            // [Line 22] Obtaining a slice from the buffer with user-supplied length.
            let slice = std::slice::from_raw_parts(self.buffer.as_ptr(), len);
            // [Line 23] Converting to &str without verifying valid UTF-8 or bounds.
            let s = std::str::from_utf8_unchecked(slice);
            // [Line 24] Returning the string, potentially reading out‐of‐bounds.
            Ok(s.to_string())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <length>", args[0]);
        std::process::exit(1);
    }
    let n: usize = match args[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid length provided");
            std::process::exit(1);
        }
    };

    let processor = Arc::new(Processor::new());
    let proc_clone = Arc::clone(&processor);

    let handle = thread::spawn(move || {
        match proc_clone.process(n) {
            Ok(s) => println!("Thread Output: {}", s),
            Err(e) => eprintln!("Thread Error: {}", e),
        }
    });

    match processor.process(n) {
        Ok(s) => println!("Main Output: {}", s),
        Err(e) => eprintln!("Main Error: {}", e),
    }

    handle.join().unwrap();
}