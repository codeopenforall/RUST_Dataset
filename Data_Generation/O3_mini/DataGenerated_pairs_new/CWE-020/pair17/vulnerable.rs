//////////////////////////////
// Vulnerable Code for Improper Input Validation (CWE-020)
// This code compiles as an independent executable.
// It uses unsafe blocks and concurrency to process a string slice
// based on a user-supplied length without validating that the length
// does not exceed the actual string length.
use std::env;
use std::sync::Arc;
use std::thread;

pub struct Processor {
    data: String,
}

impl Processor {
    pub fn new(data: String) -> Self {
        Processor { data }
    }
    
    // This function uses an unsafe block to convert a subslice into a &str.
    // It does not verify that the user length is less than or equal to data length,
    // which may lead to a panic (or worse in a different context) when slicing.
    pub fn process(&self, user_len: usize) -> Result<&str, &'static str> {
        let bytes = self.data.as_bytes();
        // Vulnerability: no validation of user_len against bytes.len()
        unsafe {
            Ok(std::str::from_utf8_unchecked(&bytes[..user_len]))
        }
    }
    
    pub fn run(self, user_len: usize) {
        let shared = Arc::new(self);
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // If user_len is too large, the slicing will panic.
            // The result is then unwrapped to simulate using the value directly.
            // In a real-world scenario this could lead to a denial-of-service.
            let result = thread_shared.process(user_len).unwrap();
            println!("Processed data: {}", result);
        });
        handle.join().unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_string> <length>", args[0]);
        std::process::exit(1);
    }
    let input = args[1].clone();
    let user_len: usize = args[2].parse().unwrap();
    
    let proc_inst = Processor::new(input);
    // Running the processing in a separate thread.
    proc_inst.run(user_len);
}