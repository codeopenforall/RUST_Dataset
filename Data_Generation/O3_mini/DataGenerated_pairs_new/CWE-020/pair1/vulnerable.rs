//////////////// Vulnerable Version //////////////////
// This code processes an input message that starts with a 4‐byte little‐endian length header,
// followed by the message bytes. It uses unsafe blocks to convert the raw buffer into a string
// without validating that the buffer is long enough. In multi‐threaded context the processing 
// is done concurrently via a shared pointer.
use std::sync::Arc;
use std::thread;

struct Processor;

impl Processor {
    // Parses a message from input where the first 4 bytes are a little-endian u32 specifying the 
    // claimed length of the subsequent message. 
    // Vulnerability: The routine does not check that the input buffer contains at least 4 + claimed bytes.
    fn process_input(&self, input: &[u8]) -> Result<String, &'static str> {
        if input.len() < 4 {
            return Err("Input too short for header");
        }
        // Read the claimed length from the first four bytes.
        let claimed = u32::from_le_bytes([input[0], input[1], input[2], input[3]]) as usize;
        // FLAW: Not validating that input.len() >= 4 + claimed.
        let data_ptr = unsafe { input.as_ptr().add(4) };
        // Unsafe conversion: it creates a slice of length `claimed` without verifying memory safety.
        let message_slice = unsafe { std::slice::from_raw_parts(data_ptr, claimed) };
        // Bypassing UTF-8 validation.
        let result_str = unsafe { std::str::from_utf8_unchecked(message_slice) };
        Ok(result_str.to_string())
    }
}

fn concurrent_process(proc: Arc<Processor>, data: Vec<u8>) -> Option<String> {
    let handle = thread::spawn(move || {
        // Simulating additional complexity by delegating processing to a separate thread.
        proc.process_input(&data)
    });
    // Join the thread and propagate the result if available.
    handle.join().ok().and_then(|r| r.ok())
}

fn main() {
    let proc = Arc::new(Processor);
    // Here we use an input where the header claims a length of 5 with an exact message.
    // In a real attack scenario, a malicious client could supply a smaller buffer.
    let data = vec![5, 0, 0, 0, b'H', b'e', b'l', b'l', b'o'];
    match concurrent_process(proc, data) {
        Some(message) => println!("Processed message: {}", message),
        None => println!("Processing failed"),
    }
}