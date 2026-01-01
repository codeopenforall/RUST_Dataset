use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

fn run(input: &str) -> Result<String, String> {
    // Simulating unsafe processing that leads to unsanitized command injection:
    unsafe {
        // Deliberate unsafe pointer conversion to mimic real-world complexity.
        let ptr = input as *const str;
        let unsafe_input = &*ptr;
        // Construct a shell command using the unsanitized input.
        // This passes metacharacters to the shell, creating an injection vulnerability.
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {}", unsafe_input))
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err("Command executed with error".into())
        }
    }
}

fn main() {
    // Simulate a concurrent workload passing untrusted data for command execution.
    let shared_data = Arc::new(Mutex::new(String::from("Hello")));
    let data_clone = shared_data.clone();
    let handle = thread::spawn(move || {
        // No proper sanitization, enabling OS command injection.
        let result = run(&data_clone.lock().unwrap());
        match result {
            Ok(out) => println!("Output: {}", out),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}