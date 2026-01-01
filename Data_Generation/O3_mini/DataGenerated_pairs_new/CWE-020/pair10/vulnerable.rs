//////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust program simulates a configuration processor that reads an input number from the user
// and then unsafely interprets a fixed buffer as a UTF-8 string without proper bounds checking.
// It uses unsafe constructs, smart pointers (Arc), and concurrency (thread spawn) to mimic real-world issues.
// CWE-020: Improper Input Validation – the user-provided number (representing a desired slice length)
// isn’t validated against the actual buffer size, which can lead to reading unallocated memory.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::env;
use std::thread;

fn process_data(n: usize) -> Result<String, &'static str> {
    // The fixed size buffer from which we create a string slice.
    let buf: Arc<[u8; 10]> = Arc::new(*b"0123456789");

    // Spawn a thread to simulate concurrent processing.
    let shared_buf = buf.clone();
    let handle = thread::spawn(move || {
        unsafe {
            // FLAW: No validation is done to ensure 'n' does not exceed the buffer length.
            // Using from_raw_parts without bounds check can lead to memory-safety issues.
            let slice = std::slice::from_raw_parts(shared_buf.as_ptr(), n);
            std::str::from_utf8_unchecked(slice)
        }
    });

    // Join the thread and return the resulting string.
    handle.join().map(|s| s.to_string()).map_err(|_| "Thread failed")
}

fn main() {
    // The program accepts a single argument representing the desired number of bytes.
    // If no argument is provided, it defaults to 8 bytes.
    let args: Vec<String> = env::args().collect();
    let num = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(8)
    } else {
        8
    };

    // Process the input and print the result or error.
    match process_data(num) {
        Ok(s) => println!("Result: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}