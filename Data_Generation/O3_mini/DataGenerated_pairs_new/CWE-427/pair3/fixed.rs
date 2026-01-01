#![allow(unused)]
use std::env;
use std::thread;

struct Runner;

impl Runner {
    fn execute(&self) -> Result<String, String> {
        let default_safe_path = "/usr/bin:/bin".to_string();
        // Read user input from "SEARCH_PATH" with a fallback to a safe default.
        let user_input = env::var("SEARCH_PATH").unwrap_or_else(|_| default_safe_path.clone());

        // Sanitize: Only allow absolute paths (starting with '/') for each component.
        let sanitized: Vec<&str> = user_input.split(':').filter(|segment| segment.starts_with('/')).collect();
        let safe_value = if sanitized.is_empty() {
            default_safe_path
        } else {
            sanitized.join(":")
        };

        // Concurrency: Spawn a thread that sets the "PATH" variable safely.
        let safe_clone = safe_value.clone();
        let handle = thread::spawn(move || {
            env::set_var("PATH", safe_clone);
        });
        handle.join().unwrap();

        // Return the current PATH value.
        let current_path = env::var("PATH").unwrap_or_default();
        Ok(current_path)
    }
}

fn main() {
    let runner = Runner;
    match runner.execute() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}