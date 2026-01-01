////////////////////////////////////////////////////////////////////////////////
// This code simulates a scenario where an external executable is launched 
// using a relative command name (e.g. "untrusted_binary") without validating 
// its absolute location. An adversary controlling the search path (via PATH 
// environment variable) could substitute a malicious binary. The code uses 
// unsafe blocks and concurrent threads to mimic a real-world application.
// Note: A helper function (check_command) is provided to let external tests 
// trigger the vulnerability.
////////////////////////////////////////////////////////////////////////////////

use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

struct Launcher {
    binary: String,
}

impl Launcher {
    fn new(input: &str) -> Self {
        Self { binary: input.to_string() }
    }

    // Executes the external command, intended to return the version string.
    // Uses an unsafe block to emulate low-level interactions.
    fn run(&self) -> Result<String, String> {
        let mut output = String::new();
        let result = unsafe {
            // Vulnerability: using a relative path can lead to loading an 
            // unintended executable from an untrusted directory.
            Command::new(&self.binary)
                .arg("--version")
                .output()
        };
        match result {
            Ok(o) => {
                output = String::from_utf8_lossy(&o.stdout).to_string();
                Ok(output)
            }
            Err(e) => Err(format!("Error launching command: {}", e))
        }
    }
}

// This public helper allows external tests (oracle) to trigger the behavior.
pub fn check_command(input: &str) -> Result<String, String> {
    let instance = Launcher::new(input);
    instance.run()
}

fn execute_threads() -> i32 {
    let shared = Arc::new(Mutex::new(Launcher::new("untrusted_binary")));
    let mut handles = vec![];

    // Spawn two threads running the command concurrently.
    for _ in 0..2 {
        let thread_launcher = shared.clone();
        let handle = thread::spawn(move || {
            let guard = thread_launcher.lock().unwrap();
            let _ = guard.run();
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    0
}

fn main() {
    // In the vulnerable version, the relative path "untrusted_binary" is used.
    std::process::exit(execute_threads());
}