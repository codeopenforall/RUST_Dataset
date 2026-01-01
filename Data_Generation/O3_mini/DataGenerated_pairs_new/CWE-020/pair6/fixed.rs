////////////////////////////////////////////////////////////
// Corrected Rust Code (Proper Input Validation for CWE-020)
////////////////////////////////////////////////////////////

use std::env;
use std::str;
use std::thread;

struct Processor;

impl Processor {
    // This function processes the input safely by verifying that the provided length
    // does not exceed the actual payload size and performs a safe UTF-8 conversion.
    fn process(&self, input: &str) -> Result<String, String> {
        let mut parts = input.splitn(2, ' ');
        let len_str = parts.next().ok_or("Missing length token")?;
        let data = parts.next().ok_or("Missing payload token")?;
        let expected_len: usize = len_str.parse().map_err(|_| "Invalid length token")?;
        // Validate that the expected length is not greater than the payload length.
        if expected_len > data.len() {
            return Err("Length exceeds payload size".to_owned());
        }
        // Safely slice the payload and convert it using the safe from_utf8 function.
        let subslice = &data.as_bytes()[..expected_len];
        let result = str::from_utf8(subslice).map_err(|_| "Payload is not valid UTF-8")?;
        Ok(result.to_owned())
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