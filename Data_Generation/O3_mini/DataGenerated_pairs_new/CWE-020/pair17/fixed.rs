//////////////////////////////
// Fixed Code for Improper Input Validation (CWE-020)
// This corrected version compiles as an independent executable.
// It fixes the vulnerability by validating that the user-supplied length does not exceed
// the actual data length before using an unsafe conversion to a &str.
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
    
    // Corrected function that checks the user length.
    // If the length is greater than the actual data length, it returns an error
    // rather than attempting an unsafe slice conversion.
    pub fn process(&self, user_len: usize) -> Result<&str, &'static str> {
        let bytes = self.data.as_bytes();
        if user_len > bytes.len() {
            return Err("Input length exceeds data length");
        }
        unsafe {
            Ok(std::str::from_utf8_unchecked(&bytes[..user_len]))
        }
    }
    
    pub fn run(self, user_len: usize) {
        let shared = Arc::new(self);
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // This call now returns an error for an invalid length instead of panicking.
            match thread_shared.process(user_len) {
                Ok(result) => println!("Processed data: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
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
    proc_inst.run(user_len);
}