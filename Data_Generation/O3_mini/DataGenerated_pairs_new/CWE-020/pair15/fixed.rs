///////////////////////////////////////////////
// Fixed Version - CWE-020 Example
///////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Config {
    data: String,
}

impl Config {
    // Processes the input string by reading a two-digit header that
    // indicates the length of the subsequent payload.
    // Proper input validation is applied and unsafe code is avoided.
    fn parse(input: &str) -> Result<Self, &'static str> {
        if input.len() < 2 {
            return Err("Input too short");
        }
        let header = &input[..2];
        let count: usize = header.parse().map_err(|_| "Header parse error")?;
        if input.len() < 2 + count {
            return Err("Payload length mismatch");
        }
        // Validate proper UTF-8 conversion.
        let slice = &input.as_bytes()[2..2 + count];
        let payload = std::str::from_utf8(slice).map_err(|_| "Invalid UTF-8 in payload")?;
        Ok(Config {
            data: payload.to_string(),
        })
    }

    // Business logic using concurrency and smart pointers.
    fn process(&self) {
        let shared = Arc::new(self.data.clone());
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            println!("Processed data: {}", shared_clone);
        });
        handle.join().unwrap();
    }
}

fn main() {
    // Read input from command-line args; default is now safe.
    let input = std::env::args().nth(1).unwrap_or_else(|| "05hello".to_string());
    match Config::parse(&input) {
        Ok(cfg) => {
            cfg.process();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}