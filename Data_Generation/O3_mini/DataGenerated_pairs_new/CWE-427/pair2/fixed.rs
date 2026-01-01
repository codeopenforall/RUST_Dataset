//////////////////////////////
// Secure Example Code
//////////////////////////////
use std::env;
use std::process::Command;
use std::thread;

fn run_protected() {
    // Read the command name from the environment variable.
    // Default to "echo" if not set.
    let command_input = env::var("CMD_NAME").unwrap_or_else(|_| "echo".to_string());

    // Validate the command against an allowlist.
    let allowed_commands = ["echo", "safe"];
    if !allowed_commands.contains(&command_input.as_str()) {
        panic!("Command not allowed");
    }

    // Use safe Rust string handling without unsafe conversions.
    let shell = command_input.as_str();

    // Instead of using an attacker-controlled PATH, enforce a secure search path.
    let secure_path = "/usr/bin:/bin";

    // Spawn a thread for concurrent execution.
    let handler = thread::spawn(move || {
        let output = Command::new(shell)
            .env("PATH", secure_path)
            .arg("fixed")
            .output()
            .expect("failed to execute command");
        output
    });

    let result = handler.join().unwrap();
    println!("Output: {:?}", result);
}

fn main() {
    run_protected();
}