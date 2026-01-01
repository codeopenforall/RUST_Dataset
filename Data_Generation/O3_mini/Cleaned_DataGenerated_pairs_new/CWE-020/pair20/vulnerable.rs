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
        // UNSAFE USAGE: Intentionally using pointer arithmetic and unchecked UTF-8 conversion.
        // Vulnerability: No validation is done to ensure len is within the bounds of the buffer.
        let ptr = self.buffer.as_ptr();                                     // [16]
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };         // [17]
        let result = unsafe { str::from_utf8_unchecked(slice) };
        result.to_string()
    }
}
fn main() {
    // Read command line argument; default will be "15" to trigger the vulnerability.
    let arg = env::args().nth(1).unwrap_or_else(|| "15".to_string());
    let proc_inst = Arc::new(Processor::new());
    // Spawn a thread to simulate concurrent processing.
    let proc_clone = Arc::clone(&proc_inst);
    let handle = thread::spawn(move || {
        // This call can invoke undefined behavior if input length exceeds the allocated buffer.
        let output = proc_clone.process(&arg);
        println!("Output: {}", output);
    });
    handle.join().expect("Thread panicked");
}
