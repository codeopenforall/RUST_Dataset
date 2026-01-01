////////////////////////////////////////////////////////////
// Vulnerability Example for CWE-427: Uncontrolled Search Path Element
// This code reads an environment variable "MALICIOUS_PATH" from an untrusted source,
// unsafely converts it using an unsafe block, and uses its content to override the PATH
// environment variable for a spawned command. An attacker controlling the variable may
// cause execution of malicious binaries.
////////////////////////////////////////////////////////////

use std::env;
use std::process::Command;
use std::thread;

trait Executor {
    fn launch(&self) -> Result<(), std::io::Error>;
}

// This structure represents an application that uses the untrusted PATH value.
struct Application;

impl Application {
    // This helper function unsafely resolves the PATH value from an environment variable.
    // It does not perform any validation.
    fn resolve_path(&self) -> String {
        let custom = env::var("MALICIOUS_PATH").unwrap_or_default();
        // Unsafe conversion without UTF-8 checks (mimics unsafe real-world pattern)
        unsafe {
            let ptr = custom.as_ptr();
            let len = custom.len();
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice).to_string()
        }
    }
}

impl Executor for Application {
    fn launch(&self) -> Result<(), std::io::Error> {
        let chosen_path = self.resolve_path();
        let mut cmd = Command::new("ls");
        // Use the attacker-controlled string to set the PATH environment variable.
        if !chosen_path.is_empty() {
            cmd.env("PATH", chosen_path);
        }
        // Spawn the command in a separate thread.
        let handler = thread::spawn(move || {
            // Execute the command, output is discarded.
            cmd.output().expect("failed to run process")
        });
        handler.join().unwrap();
        Ok(())
    }
}

fn main() {
    let app = Application;
    // Launch the process; any error is printed.
    if let Err(e) = app.launch() {
        eprintln!("Error occurred: {:?}", e);
    }
}