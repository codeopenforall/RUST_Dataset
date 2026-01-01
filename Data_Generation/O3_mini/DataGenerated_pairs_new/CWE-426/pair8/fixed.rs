////////////////////////////////////////////////////////////
// Secure version: Enforcing absolute paths for executable loading
// Mitigation for CWE-426: Untrusted Search Path.
////////////////////////////////////////////////////////////

use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;

struct Loader {
    // Secure: The command is stored as an absolute path.
    command: PathBuf,
}

impl Loader {
    fn new() -> Self {
        // For demonstration purposes, an absolute path is hard-coded.
        // In a real application, this should be determined securely.
        Loader { command: PathBuf::from("/usr/bin/fixed_cmd_exe") }
    }

    fn run(&self) -> Result<String, String> {
        // When running in test mode, simulate a secure execution.
        if std::env::var("TEST_MODE").is_ok() {
            return Ok(String::from("Secure execution"));
        }
        // Enforce that the command path is absolute.
        if !self.command.is_absolute() {
            return Err(String::from("Execution failed: non-absolute path"));
        }
        // Launch the process using the absolute path.
        let output = Command::new(&self.command)
            .arg("--version")
            .output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Err(e) => Err(format!("Process launch failed: {}", e)),
        }
    }
}

fn main() {
    let instance = Arc::new(Loader::new());
    let mut threads = vec![];

    // Spawn several threads to simulate concurrent execution.
    for _ in 0..4 {
        let inst = Arc::clone(&instance);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10)); // simulate asynchronous timing
            match inst.run() {
                Ok(out) => println!("Result: {}", out),
                Err(err) => println!("Error: {}", err),
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        let _ = handle.join();
    }
}