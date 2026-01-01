#!/usr/bin/env rust
// Vulnerable application: uses unsafe pointer conversion and shell command construction that is prone to injection.
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::env;

pub fn run_command(input: &str) -> Result<String, String> {
    // Build a shell command string by directly concatenating untrusted input.
    // CWE-088: OS Command Injection vulnerability – unsanitized input is fed to "sh -c".
    let command_string = format!("echo {}", input);
    unsafe {
        // UNSAFE: Creating a new String from raw parts of an existing String.
        // This simulates unsafe patterns and introduces the possibility of memory unsafety.
        let ptr = command_string.as_ptr();
        let len = command_string.len();
        let capacity = command_string.capacity();
        let cmd = String::from_raw_parts(ptr as *mut u8, len, capacity);
        // Spawn a thread to execute the command concurrently.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            // The command is executed using "sh -c" with the unsanitized cmd string.
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output();
            tx.send(output).unwrap();
        });
        match rx.recv() {
            Ok(Ok(o)) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Ok(Err(e)) => Err(format!("Command error: {:?}", e)),
            Err(e) => Err(format!("Channel error: {:?}", e)),
        }
    }
}

fn main() {
    // Read input from command-line argument; defaults to "Hello" if none provided.
    let input = env::args().nth(1).unwrap_or_else(|| "Hello".to_string());
    match run_command(&input) {
        Ok(out) => println!("Result: {}", out),
        Err(err) => eprintln!("Error: {}", err),
    }
}