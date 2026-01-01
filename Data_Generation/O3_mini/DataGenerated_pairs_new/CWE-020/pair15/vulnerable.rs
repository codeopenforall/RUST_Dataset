///////////////////////////////////////////////
// Vulnerable Version - CWE-020 Example
///////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Config {
    data: String,
}

impl Config {
    // Processes the input string by reading a two-digit header that
    // indicates the length of the subsequent payload.
    // Unsafe code is used to skip bounds-checking, making it vulnerable
    // to malformed input.
    fn parse(input: &str) -> Result<Self, &'static str> {
        // Expect at least 2 bytes for the header.
        if input.len() < 2 {
            return Err("Input too short");
        }
        let header = &input[..2];
        // Unchecked conversion: panic possible if header isn't numeric.
        let count: usize = header.parse().unwrap();
        let bytes = input.as_bytes();
        unsafe {
            // Vulnerability: not verifying that (2 + count) is within bytes bounds.
            let slice = bytes.get_unchecked(2..2 + count);
            // Unsafe conversion from bytes to str without UTF-8 validation.
            let payload = std::str::from_utf8_unchecked(slice);
            Ok(Config {
                data: payload.to_string(),
            })
        }
    }

    // A sample business logic using concurrency and smart pointers.
    fn process(&self) {
        let shared = Arc::new(self.data.clone());
        let shared_clone = Arc::clone(&shared);
        // Spawn a thread to simulate concurrent processing of the payload.
        let handle = thread::spawn(move || {
            // Do some work with the shared data.
            println!("Processed data: {}", shared_clone);
        });
        handle.join().unwrap();
    }
}

fn main() {
    // Read input from command-line args; default triggers vulnerability.
    let input = std::env::args().nth(1).unwrap_or_else(|| "10short".to_string());
    // The input is expected to have first two digits as a length header,
    // followed by payload. If the header indicates a length larger than available data,
    // the unsafe code will access out-of-bounds.
    match Config::parse(&input) {
        Ok(cfg) => {
            cfg.process();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}