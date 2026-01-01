////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust code provides a corrected implementation that defends against OS command injection
// (CWE-088) by validating the provided input. The code rejects input that contains dangerous
// metacharacters, thereby preventing command injection. The overall design (using Arc, Mutex, threads)
// remains as in the vulnerable version.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

struct Executor {
    content: String,
}

impl Executor {
    fn new(content: String) -> Self {
        Executor { content }
    }

    // Validates the input to ensure it does not contain shell metacharacters.
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Reject input containing characters that could modify command behavior.
        let forbidden = [';', '&', '|', '$', '`', '>', '<'];
        if self.content.chars().any(|c| forbidden.contains(&c)) {
            return Err("Input contains disallowed characters".into());
        }
        Ok(())
    }

    // Secure execution: the function first validates the input then builds the command.
    fn run(&self) -> Result<String, Box<dyn Error>> {
        self.validate()?;

        // Safe reconstruction of the input. Even though we still use an unsafe block,
        // it is only used to mimic realistic patterns and does not contribute to the vulnerability.
        let ptr = self.content.as_ptr();
        let len = self.content.len();
        let safe_input = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        };
        // Build the command after validation.
        let cmd_str = format!("echo safe && {}", safe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

fn main() {
    // Accept input from the first command-line argument; if absent, use "default".
    let input = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let executor = Executor::new(input);
    let shared_exe = Arc::new(Mutex::new(executor));

    // Spawn a thread that executes the command.
    let shared_clone = Arc::clone(&shared_exe);
    let handle = thread::spawn(move || {
        let exe = shared_clone.lock().unwrap();
        match exe.run() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("execution error: {}", e);
                "error".to_string()
            }
        }
    });

    let result = handle.join().unwrap();
    println!("{}", result);
}