//////////////////////////////////////////////////////////////
// Real-world style code exhibiting CWE-020: Improper Input
// Validation - Unsafe unchecked pointer arithmetic.
// This example builds a configuration processor that uses a
// user-supplied index to extract an element from a vector.
// It unsafely dereferences the pointer without validating that
// the index is within bounds, allowing an attacker to trigger
// an out‐of‐bounds access.
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
    // UNSAFE: Does not validate that the provided index is within the bounds of 'data'.
    unsafe fn process(&self, input: &str) -> Result<u32, String> {
        // Attempt to parse the user-provided string to an index.
        let idx: usize = input.trim().parse().map_err(|_| "Invalid input: not a number".to_string())?;
        // FLAW: No check is performed to ensure that 'idx' is less than the length of 'data'.
        // The pointer arithmetic here is unsafely trusting the user-supplied index.
        let ptr = self.data.as_ptr();
        // Vulnerability occurs on the following line:
        let value = *ptr.add(idx);
        Ok(value)
    }
}

fn main() {
    // Accept a command-line argument as input; defaults to "2" if not provided.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "2" };
    let config = Config::new();
    // Use an unsafe block to call the vulnerable function.
    let res = unsafe { config.process(input) };
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}