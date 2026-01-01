////////////////////////////////////////////////////////////////
// Corrected Code - CWE-088 Remediation
// This corrected version eliminates the shell command injection flaw.
// It avoids using a shell ("sh -c") and instead executes the command directly by passing
// the untrusted input as a separate argument. This prevents shell metacharacters from being expanded.
////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

struct Executor {
    payload: Arc<UnsafeCell<String>>,
}

// Even though UnsafeCell is used, we control the usage so we manually assert Sync.
unsafe impl Sync for Executor {}

impl Executor {
    // Creates a new instance using the provided input.
    fn new(data: &str) -> Self {
        Executor {
            payload: Arc::new(UnsafeCell::new(data.to_string())),
        }
    }
    
    // Executes a command safely by avoiding shell interpolation.
    // Instead of using format! with "sh -c", it calls "echo" directly with the user input
    // as an argument. This prevents the input from being interpreted by a shell.
    fn execute(&self) -> String {
        let shared = self.payload.clone();
        let unsafe_input = unsafe { &*shared.get() };
        // FIXED: Directly pass the argument to "echo" to avoid shell meta expansion.
        let output = Command::new("echo")
            .arg(unsafe_input)
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
    // Even if the input contains shell metacharacters, they will not be interpreted.
    let result = process("safe; echo injected");
    println!("{}", result);
}