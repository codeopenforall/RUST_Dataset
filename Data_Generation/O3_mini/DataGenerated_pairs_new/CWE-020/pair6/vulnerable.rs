////////////////////////////////////////////////////////////
// Vulnerable Rust Code (CWE-020: Improper Input Validation)
////////////////////////////////////////////////////////////

use std::env;
use std::str;
use std::slice;
use std::thread;

struct Processor;

impl Processor {
    // This function processes a space‐separated input:
    // The first token is expected to be a numeric length, and the remainder is the payload.
    // It then unsafely converts a slice of the payload into a string using the provided length.
    // WARNING: No bounds-check is performed on the length against the actual payload size.
    fn process(&self, input: &str) -> Result<String, String> {
        let mut parts = input.splitn(2, ' ');
        let len_str = parts.next().ok_or("Missing length token")?;
        let data = parts.next().ok_or("Missing payload token")?;
        let expected_len: usize = len_str.parse().map_err(|_| "Invalid length token")?;
        // The vulnerability: using unsafe code with unchecked pointer arithmetic based on user input.
        unsafe {
            // Directly create a slice from the raw pointer without verifying that expected_len is within bounds.
            let ptr = data.as_ptr();
            let unslice = slice::from_raw_parts(ptr, expected_len);
            // Convert to string without checking UTF-8 validity.
            let result = str::from_utf8_unchecked(unslice);
            Ok(result.to_owned())
        }
    }
}

fn main() {
    let processor = Processor;
    let args: Vec<String> = env::args().collect();
    // Expecting arguments: <expected_length> <payload>
    if args.len() < 3 {
        println!("Usage: {} <expected_length> <payload>", args[0]);
        return;
    }
    // Combine arguments so that spaces in the payload are preserved.
    let input = format!("{} {}", args[1], args[2]);
    // Run the processing concurrently in a separate thread.
    let handle = thread::spawn(move || {
        match processor.process(&input) {
            Ok(out)  => println!("Output: {}", out),
            Err(err) => println!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}