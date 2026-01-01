use std::env;
use std::sync::{Arc};
use std::thread;
use std::str;
struct Processor {
    buffer: Vec<u8>,
}
impl Processor {
    fn new() -> Self {
        Processor { buffer: b"Sensitive buffer data".to_vec() }
    }
    fn process(&self, len: usize) -> Result<String, &'static str> {
        if len > self.buffer.len() {
            return Err("Invalid length: exceeds buffer size");
        }
        let s = std::str::from_utf8(&self.buffer[..len]).map_err(|_| "UTF8 conversion error")?;
        Ok(s.to_string())
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
