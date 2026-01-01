//////////////// Corrected Code ////////////////////
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::thread;

struct DataProcessor;

impl DataProcessor {
    // This function safely validates that the input has a proper 4-character length field,
    // confirms that the declared length does not exceed the available data,
    // and then converts the data slice using a safe UTF‑8 check.
    fn process(&self, input: &str) -> Result<String, Box<dyn Error>> {
        if input.len() < 4 {
            return Err("Input too short to contain length field".into());
        }
        let len_str = &input[..4];
        let len: usize = len_str.parse()?;
        // Validate that the input contains enough characters.
        if input.len() < 4 + len {
            return Err("Declared length exceeds input size".into());
        }
        let data_slice = &input[4..4 + len];
        // Use safe conversion which checks that the data is valid UTF-8.
        let result = std::str::from_utf8(data_slice.as_bytes())?;
        Ok(result.to_string())
    }
}

fn main() {
    // Expect input provided via command line argument.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }
    let input = args[1].as_str();
    let shared = Arc::new(DataProcessor);
    let mut threads = vec![];
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let input_clone = input.to_string();
        let handle = thread::spawn(move || {
            match shared_clone.process(&input_clone) {
                Ok(result) => println!("Output: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}