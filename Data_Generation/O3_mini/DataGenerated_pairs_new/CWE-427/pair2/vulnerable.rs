//////////////////////////////
// Vulnerability Example Code
//////////////////////////////
use std::env;
use std::process::Command;
use std::thread;

fn run_action() {
    // Read a command name from an environment variable.
    // This variable is attacker-controlled and can be set to an unexpected value.
    let command_input = env::var("CMD_NAME").unwrap_or_else(|_| "echo".to_string());

    // UNSAFE: Convert the command string via raw pointer manipulation.
    // This unsafe block simulates a common unsafe pattern seen in some real-world cases,
    // where unchecked memory operations and unsanitized input lead to potential vulnerabilities.
    let shell = unsafe {
        let raw_ptr = command_input.as_ptr();
        let len = command_input.len();
        // Using from_utf8_unchecked to bypass input validation
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, len))
    };

    // Acquire the PATH environment variable directly;
    // if an attacker manipulates PATH, a malicious binary might be resolved.
    let insecure_path = env::var("PATH").unwrap_or_default();

    // Spawn a thread to simulate concurrent execution and potential race conditions.
    let handler = thread::spawn(move || {
        // Using the attacker-controlled environment PATH without sanitization.
        let output = Command::new(shell)
            .env("PATH", insecure_path)
            .arg("vulnerable")
            .output()
            .expect("failed to execute command");
        output
    });

    let result = handler.join().unwrap();
    println!("Output: {:?}", result);
}

fn main() {
    run_action();
}