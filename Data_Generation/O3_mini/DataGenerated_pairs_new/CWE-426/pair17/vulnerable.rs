// Vulnerable sample for CWE-426: Untrusted Search Path.
// This code uses an unsafe block and spawns a thread to execute a command using a
// relative path provided during instantiation. An attacker might arrange for a malicious
// executable named "vulnerable_app" to be found in the untrusted library search path.
// (Note: This sample mimics realistic patterns found in real-world RustSec issues.)
use std::process::Command;
use std::thread;
use std::sync::Arc;

trait Executor {
    fn execute(&self) -> Result<(), String>;
}

struct Launcher {
    path: String,
}

impl Executor for Launcher {
    fn execute(&self) -> Result<(), String> {
        // Using unsafe to simulate a low-level FFI scenario.
        unsafe {
            // Vulnerability: Using a relative path without validation. An attacker can alter
            // the search order (e.g., by placing a malicious executable in a directory that appears first).
            let status = Command::new(&self.path)
                .arg("--version")
                .status()
                .map_err(|e| e.to_string())?;
            if status.success() {
                // In a vulnerable scenario, a relative command might have unexpectedly executed malicious code.
                Ok(())
            } else {
                Err("Command execution failed".to_owned())
            }
        }
    }
}

fn run() {
    let launcher = Arc::new(Launcher { path: "vulnerable_app".to_owned() });
    let launcher_clone = Arc::clone(&launcher);
    let handle = thread::spawn(move || {
        // In an actual environment, the untrusted search path might lead to an unexpected binary.
        let _ = launcher_clone.execute();
    });
    handle.join().unwrap();
}

fn main() {
    run();
}