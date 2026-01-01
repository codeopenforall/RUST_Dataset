/*
 * This program implements a command executor that spawns a thread to run OS commands.
 * It uses an unsafe block to manipulate a raw pointer of the input string before
 * constructing a shell command. The input is interpolated into a shell command without
 * proper sanitization, paving the way for OS command injection (CWE-088).
 */

use std::process::Command;
use std::env;
use std::thread;
use std::io;

pub trait CommandExecutor {
    fn execute(&self, input: String) -> io::Result<String>;
}

pub struct Worker;

impl CommandExecutor for Worker {
    fn execute(&self, cmd: String) -> io::Result<String> {
        // Unsafe pointer conversion simulating complex unsafe usage.
        // Vulnerable: using the raw pointer to reconstruct the String without validation.
        let unsafe_input = unsafe {
            let raw_ptr = cmd.as_ptr();
            let len = cmd.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).into_owned()
        };

        // Constructing a shell command by directly embedding untrusted input.
        // This results in OS command injection vulnerabilities when the input contains
        // shell metacharacters.
        let full_command = format!("echo {}", unsafe_input);

        // Execute using shell invocation, allowing subsequent commands if injected.
        let output = Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn run_concurrently(input: String) -> io::Result<String> {
    let worker = Worker;
    let handler = thread::spawn(move || worker.execute(input));
    // In a real application, proper error handling for join errors is required.
    handler.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Use the first command-line argument if available; otherwise, use a default string.
    let user_data = if args.len() > 1 { args[1].clone() } else { String::from("default_input") };

    match run_concurrently(user_data) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}