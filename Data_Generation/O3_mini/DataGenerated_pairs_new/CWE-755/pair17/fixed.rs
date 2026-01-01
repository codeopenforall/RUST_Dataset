/////////////////////////////////////////////////////////////////
// Corrected Code Sample - Proper Error Propagation
/////////////////////////////////////////////////////////////////

use std::env;
use std::sync::Arc;
use std::thread;

struct Config {
    value: i32,
}

impl Config {
    // Safe conversion that returns an error on failure.
    fn from_str(data: &str) -> Result<Self, String> {
        data.trim()
            .parse::<i32>()
            .map(|val| Config { value: val })
            .map_err(|e| format!("Parsing error: {}", e))
    }
}

fn process_input(input: String) -> Result<i32, String> {
    let data = Arc::new(input);
    let data_clone = Arc::clone(&data);
    // Spawn a thread to simulate concurrent processing.
    let handle = thread::spawn(move || {
        // An unsafe block is used only to mimic a similar structure.
        // The inner function now safely propagates parsing errors.
        unsafe { Config::from_str(data_clone.as_str()) }
    });
    // Propagate any errors from the thread or the parsing.
    let config = handle.join().map_err(|_| "Thread panicked".to_string())??;
    Ok(config.value)
}

fn main() {
    // Retrieve input from commandline arguments; default to "bad" if none provided.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "bad".to_string()
    };

    match process_input(input) {
        Ok(val) => {
            println!("Computed value: {}", val);
            // A computed value of 0 is now treated as a failure.
            std::process::exit(if val != 0 { 0 } else { 1 });
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}