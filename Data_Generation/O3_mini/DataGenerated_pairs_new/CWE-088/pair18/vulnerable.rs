////////////////////////////////////////////////////////////////
// Vulnerable Code - CWE-088 (OS Command Injection)
// This code uses unsafe pointer dereferencing and executes a shell
// command by interpolating untrusted input into a command string.
// The use of "sh -c" together with format! leads to command injection.
////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

struct Executor {
    payload: Arc<UnsafeCell<String>>,
}

// UNSAFE: Manually asserting Sync for a type containing UnsafeCell.
unsafe impl Sync for Executor {}

impl Executor {
    // Creates a new instance with the provided input.
    fn new(data: &str) -> Self {
        Executor {
            payload: Arc::new(UnsafeCell::new(data.to_string())),
        }
    }
    
    // Executes a shell command using a vulnerable pattern:
    // It unsafely reads untrusted input via a raw pointer,
    // interpolates it into a command string, and calls "sh -c".
    // CWE-088: Passing unsanitized input to a shell.
    fn execute(&self) -> String {
        let shared = self.payload.clone();
        // Unsafely dereference raw pointer to get the input string.
        let unsafe_input = unsafe { &*shared.get() };
        // VULNERABLE: Untrusted input is injected into a shell command.
        let cmd = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

/// Public interface for testing. Accepts an input string,
/// constructs an Executor, and returns the output from execution.
pub fn process(input: &str) -> String {
    let executor = Executor::new(input);
    executor.execute()
}

fn main() {
    // Using a malicious input that contains a shell metacharacter.
    // In this vulnerable code, the semicolon leads to execution of an extra command.
    let result = process("safe; echo injected");
    println!("{}", result);
}