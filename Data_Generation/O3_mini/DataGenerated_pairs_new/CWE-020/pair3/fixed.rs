//////////////////////////////////////////////////////////////
// Revised code addressing CWE-020 by validating user input.
// The configuration processor now checks that the parsed index is
// within the valid bounds before performing unsafe operations.
//////////////////////////////////////////////////////////////
use std::env;

struct Config {
    // A sample configuration data array.
    data: Vec<u32>,
}

impl Config {
    fn new() -> Self {
        // A realistic configuration with five numerical values.
        Self { data: vec![10, 20, 30, 40, 50] }
    }

    // Processes an input string by interpreting it as an index.
    // Now properly validates the index before performing pointer arithmetic.
    unsafe fn process(&self, input: &str) -> Result<u32, String> {
        // Parse the input string to an index.
        let idx: usize = input.trim().parse().map_err(|_| "Invalid input: not a number".to_string())?;
        // FIX: Validate that the index is within the bounds of the data.
        if idx >= self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        let ptr = self.data.as_ptr();
        // Safe because the index is verified.
        let value = *ptr.add(idx);
        Ok(value)
    }
}

fn main() {
    // Accept a command-line argument as input; defaults to "2" if not provided.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "2" };
    let config = Config::new();
    // Use an unsafe block for the function call, now safe due to proper validation.
    let res = unsafe { config.process(input) };
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}