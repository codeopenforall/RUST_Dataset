use std::env;
use std::sync::Arc;
use std::thread;
use std::str;
struct Processor {
    buffer: Vec<u8>,
}
impl Processor {
    pub fn new() -> Self {
        Processor { buffer: vec![b'A'; 10] }
    }
    pub fn process(&self, input: &str) -> String {
        // Parse the user-controlled input as a length value.
        let len: usize = input.parse().expect("Failed to parse input");
        // FIX: Validate that the provided length does not exceed the buffer size.
        if len > self.buffer.len() {
            return "Invalid input size".to_string();
        }
        // Safe conversion: Using pointer arithmetic only after ensuring bounds.
        let ptr = self.buffer.as_ptr();
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        let result = unsafe { str::from_utf8_unchecked(slice) };
        result.to_string()
    }
}
fn main() {
    // Read command line argument; default will be "15" which is now handled safely.
    let arg = env::args().nth(1).unwrap_or_else(|| "15".to_string());
    let proc_inst = Arc::new(Processor::new());
    // Spawn a thread to simulate concurrent processing.
    let proc_clone = Arc::clone(&proc_inst);
    let handle = thread::spawn(move || {
        let output = proc_clone.process(&arg);
        println!("Output: {}", output);
    });
    handle.join().expect("Thread panicked");
}
