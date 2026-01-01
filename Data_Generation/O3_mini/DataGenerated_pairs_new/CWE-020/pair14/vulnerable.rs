//////////////// Vulnerable Code ////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

struct DataProcessor;

impl DataProcessor {
    // This function expects the input to begin with a 4‐digit number (as a string)
    // representing the declared length of the following data.
    // It unsafely slices the input without verifying that the input is long enough
    // and uses an unsafe conversion from bytes to UTF‑8.
    fn process(&self, input: &str) -> String {
        // Ensure the input has at least 4 characters for the length field.
        if input.len() < 4 {
            return String::new();
        }
        // Extract the first 4 characters as the length string.
        let len_str = &input[..4];
        // Improperly trust the parse result without bounds-checking.
        let len: usize = len_str.parse().unwrap();
        // UNSAFE: Slices without ensuring that input has enough bytes.
        let data_slice = &input[4..4 + len];
        // UNSAFE: Converts the byte slice to a string without UTF-8 validation.
        let result = unsafe { std::str::from_utf8_unchecked(data_slice.as_bytes()) };
        result.to_string()
    }
}

fn main() {
    // Expect input via command line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let input = args[1].as_str();
    let shared = Arc::new(DataProcessor);
    let mut threads = vec![];
    // Spawn several threads to process the input concurrently.
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let input_clone = input.to_string();
        let handle = thread::spawn(move || {
            let result = shared_clone.process(&input_clone);
            println!("Output: {}", result);
        });
        threads.push(handle);
    }
    for handle in threads {
        // This join may panic if one of the threads encounters an out-of-bound slice.
        handle.join().unwrap();
    }
}