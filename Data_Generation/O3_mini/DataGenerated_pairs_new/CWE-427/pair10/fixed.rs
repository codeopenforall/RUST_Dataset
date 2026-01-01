//// Fixed Code Example for CWE-427
// This improved implementation validates the externally provided environment variable ("UNTRUSTED_PATH")
// by ensuring that each path element is an absolute path (i.e. starts with '/').
// If the input is not safe, it falls back to a secure default.
// The unsafe block has been completely removed. Concurrency and realistic API usage are maintained.
use std::env;
use std::process::Command;
use std::sync::Arc;
use std::thread;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Executor
    }

    // Validates that every directory in the PATH is absolute.
    fn is_safe_path(path: &str) -> bool {
        path.split(':').all(|p| p.starts_with('/'))
    }

    // Executes a specified command and returns its standard output as a String.
    pub fn run(&self, cmd: &str) -> std::io::Result<String> {
        let untrusted = env::var("UNTRUSTED_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        // Validate the untrusted input; if unsafe, use a secure default.
        let safe_path = if Self::is_safe_path(&untrusted) {
            untrusted
        } else {
            "/usr/bin".to_string()
        };

        // Spawn the process with a validated PATH environment variable.
        let mut command = Command::new(cmd);
        command.env("PATH", safe_path);
        let output = command.output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn main() {
    let executor = Arc::new(Executor::new());
    let exec_clone = Arc::clone(&executor);

    let handle = thread::spawn(move || {
        // Using "env" to output the process environment.
        let output = exec_clone.run("env").expect("Failed to run command");
        println!("{}", output);
    });
    handle.join().expect("Thread panicked");
}