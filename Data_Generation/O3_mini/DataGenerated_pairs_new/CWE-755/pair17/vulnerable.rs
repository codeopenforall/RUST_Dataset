/////////////////////////////////////////////////////////////////
// Vulnerable Code Sample - Improper Handling of Exceptional Conditions
/////////////////////////////////////////////////////////////////

use std::env;
use std::sync::Arc;
use std::thread;

struct Config {
    value: i32,
}

impl Config {
    // Unsafe conversion that masks conversion errors.
    unsafe fn from_str(data: &str) -> Self {
        // Deliberately hides errors using unwrap_or_default, returning 0 on failure.
        let val = data.trim().parse::<i32>().unwrap_or_default();
        Config { value: val }
    }
}

fn process_input(input: String) -> Result<i32, String> {
    // Simulate concurrent processing by using an Arc-shared pointer.
    let data = Arc::new(input);
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        // Unsafe block used to simulate a real-world scenario.
        let raw = data_clone.as_str();
        // Vulnerability: any parsing error is swallowed by unwrap_or_default.
        unsafe { Config::from_str(raw) }
    });
    let config = handle.join().map_err(|_| "Thread panicked".to_string())?;
    Ok(config.value)
}

fn main() {
    // Retrieve input from commandline arguments; default to "bad" if absent.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "bad".to_string()
    };

    match process_input(input) {
        Ok(val) => {
            println!("Computed value: {}", val);
            // For valid configuration, a non-zero value is expected.
            std::process::exit(if val != 0 { 0 } else { 1 });
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}